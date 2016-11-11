module App exposing (main)

-- EXTERNAL MODULES

import Dict

import Erl

import Html exposing (..)
import Html.App

import Json.Decode
import Json.Encode

import Material
import Material.Color as Color
import Material.Layout as Layout
import Material.Scheme

import OutMessage

import WebSocket

-- LOCAL MODULES

import Settings exposing (Settings)
import SettingsPage

import Town exposing (Town)

import TownApi

import TownPage

-- MODEL

type alias Mdl = Material.Model

type alias Model =
    { townPage : TownPage.Model
    , settingsPage : SettingsPage.Model
    , town : Town
    , settings : Settings
    , syncing : Bool
    , errorMsg : String
    , mdl : Material.Model
    }

init : Model
init =
    let
        settings = Settings.init
        town = Town.init
    in
        { town = town
        , settings = settings
        -- PAGES
        , townPage = TownPage.init town
        , settingsPage = SettingsPage.init settings
        -- STATE
        , syncing = False
        , errorMsg = ""
        , mdl = Material.model
    }

-- UPDATE

type Msg
    = Synchronize
    | UpdateTownPage TownPage.Msg
    | UpdateSettingsPage SettingsPage.Msg
    | TownServerMsg String
    | Mdl (Material.Msg Msg)

update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
    case msg of
        Synchronize ->
            { model | syncing = True } ! [townServerCmd model TownApi.getState]

        UpdateTownPage msg' ->
            TownPage.update model msg' model.townPage
                |> OutMessage.mapComponent
                   (\newTownPage -> { model | townPage = newTownPage } )
                |> OutMessage.mapCmd UpdateTownPage
                |> OutMessage.evaluateMaybe handleTownMsg Cmd.none

        UpdateSettingsPage msg' ->
            SettingsPage.update msg' model.settingsPage
                |> OutMessage.mapComponent
                   (\newSettingsPage -> { model | settingsPage = newSettingsPage } )
                |> OutMessage.mapCmd UpdateSettingsPage
                |> OutMessage.evaluateMaybe handleSettingsMsg Cmd.none

        TownServerMsg payload ->
            case Json.Decode.decodeString TownApi.msg payload of
                Err e -> { model | errorMsg = e } ! []
                Ok msg -> handleTownServerMsg msg model

        Mdl msg' ->
            Material.update msg' model

handleTownMsg : TownPage.OutMsg -> Model -> (Model, Cmd Msg)
handleTownMsg msg model =
    case msg of
        TownPage.Api apiMsg ->
            model ! [townServerCmd model apiMsg]

handleSettingsMsg : SettingsPage.OutMsg -> Model -> (Model, Cmd Msg)
handleSettingsMsg msg model =
    let settings = model.settings
    in
        case msg of
            SettingsPage.SetTownUrl url ->
                let settings = {settings | townUrl = url}
                in {model | settings = settings} ! [townServerCmd model TownApi.getState]
            SettingsPage.SetArduinoUrl url ->
                let settings = {settings | arduinoUrl = url}
                in {model | settings = settings} ! [townServerCmd model <| TownApi.setArduinoAddress url]


handleTownServerMsg : TownApi.Msg -> Model -> (Model, Cmd Msg)
handleTownServerMsg msg model =
    case msg of
        TownApi.State arduinoUrl buildingInfo ->
            let
                town = model.town
                newTown = { town | buildings = getBuildings buildingInfo }

                getBuildings = Dict.fromList << List.map buildingKeyValue
                buildingKeyValue b = ( b.buildingId , { name = b.name , lights = getLights b.lights } )

                getLights = Dict.fromList << List.map lightKeyValue
                lightKeyValue l = ( l.lightId , l.color )

                settings = model.settings
                newSettings = { settings | arduinoUrl = arduinoUrl }
            in
                { model | town = newTown , settings = newSettings } ! []

        TownApi.SetLights buildingId lights -> model ! []


townServerCmd : Model -> Json.Encode.Value -> Cmd Msg
townServerCmd model value =
    WebSocket.send (Erl.toString model.settings.townUrl) (Json.Encode.encode 0 value)

-- VIEW

view : Model -> Html Msg
view model =
    Layout.render Mdl
        model.mdl
        [ Layout.fixedHeader
        ]
        { header = header model
        , drawer = drawer model
        , tabs = ([], [])
        , main = body model
        }
           |> Material.Scheme.topWithScheme Color.Blue Color.LightGreen

header : Model -> List (Html Msg)
header model =
    [ Layout.row []
          [ Layout.title [] [ text "LEGO Town" ]
          , Layout.spacer
          , Layout.navigation []
              [ syncButton model
              ]
          ]
    ]

syncButton : Model -> Html Msg
syncButton model =
    Layout.link
        [ Layout.onClick Synchronize ]
        [ text "Sync" ]

drawer : Model -> List (Html Msg)
drawer model =
    [ Html.App.map UpdateSettingsPage <|  SettingsPage.view model.settingsPage model.settings
    ]

body : Model -> List (Html Msg)
body model =
    [ Html.App.map UpdateTownPage <| TownPage.view model.townPage model.town
    , text model.errorMsg
    ]

-- MAIN

main : Program Never
main =
    Html.App.program
        { init = (init, Cmd.none)
        , view = view
        , subscriptions = subscriptions
        , update = update
        }

subscriptions : Model -> Sub Msg
subscriptions model =
    WebSocket.listen (Erl.toString model.settings.townUrl) TownServerMsg
