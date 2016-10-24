module App exposing (main)

-- EXTERNAL MODULES

import Erl

import Html exposing (..)
import Html.App

import Material
import Material.Color as Color
import Material.Layout as Layout
import Material.Scheme

import OutMessage

import Parts exposing (Index)

import WebSocket

-- LOCAL MODULES

import Settings exposing (Settings)
import SettingsPage

import Town exposing (Town)

import TownPage

-- MODEL

type alias Mdl = Material.Model

type alias Model =
    { townPage : TownPage.Model
    , settingsPage : SettingsPage.Model
    , town : Town
    , settings : Settings
    , syncing : Bool
    , mdl : Material.Model
    }

init : Model
init =
    let
        settings = Settings.init
    in
        { townPage = TownPage.init
        , settingsPage = SettingsPage.init settings
        , town = Town.init
        , settings = settings
        , syncing = False
        , mdl = Material.model
    }

-- UPDATE

type Msg
    = Synchronize
    | UpdateTownPage TownPage.Msg
    | UpdateSettingsPage SettingsPage.Msg
    | NewTownMsg String
    | Mdl (Material.Msg Msg)

update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
    case msg of
        Synchronize ->
            { model | syncing = True } ! []

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

        NewTownMsg msg ->
            model ! []

        Mdl msg' ->
            Material.update msg' model

handleTownMsg : TownPage.OutMsg -> Model -> (Model, Cmd Msg)
handleTownMsg msg model = model ! []

handleSettingsMsg : SettingsPage.OutMsg -> Model -> (Model, Cmd Msg)
handleSettingsMsg msg model =
    let settings = model.settings
    in
        case msg of
            SettingsPage.SetTownUrl url ->
                let settings = {settings | townUrl = url}
                in {model | settings = settings} ! [WebSocket.send (Erl.toString url) "init"]
            SettingsPage.SetArduinoUrl url ->
                let settings = {settings | arduinoUrl = url}
                in {model | settings = settings} ! []

-- VIEW

view : Index -> Model -> Html Msg
view index model =
    Layout.render Mdl
        model.mdl
        [ Layout.fixedHeader
        ]
        { header = header (0::index) model
        , drawer = drawer (1::index) model
        , tabs = ([], [])
        , main = body (2::index) model
        }
           |> Material.Scheme.topWithScheme Color.Blue Color.LightGreen

header : Index -> Model -> List (Html Msg)
header index model =
    [ Layout.row []
          [ Layout.title [] [ text "LEGO Town" ]
          , Layout.spacer
          , Layout.navigation []
              [ syncButton (0::index) model
              ]
          ]
    ]

syncButton : Index -> Model -> Html Msg
syncButton index model =
    Layout.link
        [ Layout.onClick Synchronize ]
        [ text "Sync" ]

drawer : Index -> Model -> List (Html Msg)
drawer index model =
    [ Html.App.map UpdateSettingsPage <|  SettingsPage.view (0::index) model.settingsPage model.settings
    ]

body : Index -> Model -> List (Html Msg)
body index model =
    [ Html.App.map UpdateTownPage <| TownPage.view (0::index) model.townPage model.town
    ]

-- MAIN

main : Program Never
main =
    Html.App.program
        { init = (init, Cmd.none)
        , view = view []
        , subscriptions = subscriptions
        , update = update
        }

subscriptions : Model -> Sub Msg
subscriptions model =
    WebSocket.listen (Erl.toString model.settings.townUrl) NewTownMsg
