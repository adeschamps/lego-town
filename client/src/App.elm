module App exposing (main)

-- EXTERNAL MODULES

import Html exposing (..)
import Json.Decode
import Json.Encode
import Material
import Material.Color as Color
import Material.Layout as Layout
import Material.Scheme
import OutMessage
import WebSocket


-- LOCAL MODULES

import Settings
import SettingsPage
import Town
import TownApi
import TownPage


-- MODEL


type alias Mdl =
    Material.Model


type alias Model =
    { town : Town.Model
    , settings : Settings.Model
    , townPage : TownPage.Model
    , settingsPage : SettingsPage.Model
    , errorMsg : String
    , mdl : Material.Model
    }


init : Model
init =
    { town = Town.init
    , settings = Settings.init
    , townPage = TownPage.init
    , settingsPage = SettingsPage.init
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


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        Synchronize ->
            model ! [ townServerCmd model TownApi.getState ]

        UpdateTownPage msg_ ->
            TownPage.update msg_ model.townPage
                |> OutMessage.mapComponent
                    (\newTownPage -> { model | townPage = newTownPage })
                |> OutMessage.mapCmd UpdateTownPage
                |> OutMessage.evaluateMaybe handleTownMsg Cmd.none

        UpdateSettingsPage msg_ ->
            SettingsPage.update msg_ model.settingsPage
                |> OutMessage.mapComponent
                    (\newSettingsPage -> { model | settingsPage = newSettingsPage })
                |> OutMessage.mapCmd UpdateSettingsPage
                |> OutMessage.evaluateMaybe handleSettingsMsg Cmd.none

        TownServerMsg payload ->
            case Json.Decode.decodeString TownApi.msg payload of
                Err e ->
                    { model | errorMsg = e } ! []

                Ok msg ->
                    handleTownServerMsg msg model

        Mdl msg_ ->
            Material.update msg_ model


handleTownMsg : TownPage.OutMsg -> Model -> ( Model, Cmd Msg )
handleTownMsg msg model =
    case msg of
        TownPage.Api apiMsg ->
            model ! [ townServerCmd model apiMsg ]


handleSettingsMsg : SettingsPage.OutMsg -> Model -> ( Model, Cmd Msg )
handleSettingsMsg msg model =
    case msg of
        SettingsPage.SettingsMsg msg_ ->
            updateSettings msg_ model


updateSettings : Settings.Msg -> Model -> ( Model, Cmd Msg )
updateSettings msg model =
    let
        ( newSettings, outMsg ) =
            Settings.update msg model.settings

        cmd =
            case outMsg of
                Just (Settings.Api cmd_) ->
                    [ townServerCmd model cmd_ ]

                Nothing ->
                    []
    in
        { model | settings = newSettings } ! cmd


handleTownServerMsg : TownApi.Msg -> Model -> ( Model, Cmd Msg )
handleTownServerMsg msg model =
    case msg of
        TownApi.State arduinoUrl buildingInfo ->
            let
                newTown =
                    Town.update (Town.SetBuildings buildingInfo) model.town

                settings =
                    model.settings

                newSettings =
                    { settings | arduinoUrl = arduinoUrl }
            in
                { model | town = newTown, settings = newSettings } ! []

        TownApi.SetLights buildingId lights ->
            model ! []


townServerCmd : Model -> Json.Encode.Value -> Cmd Msg
townServerCmd model value =
    WebSocket.send model.settings.townUrl (Json.Encode.encode 0 value)



-- VIEW


view : Model -> Html Msg
view model =
    Layout.render Mdl
        model.mdl
        [ Layout.fixedHeader
        ]
        { header = header model
        , drawer = drawer model
        , tabs = ( [], [] )
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
    [ Html.map UpdateSettingsPage <| SettingsPage.view model.settings model.settingsPage
    ]


body : Model -> List (Html Msg)
body model =
    [ Html.map UpdateTownPage <| TownPage.view model.town model.townPage
    , text model.errorMsg
    ]



-- MAIN


main : Program Never Model Msg
main =
    Html.program
        { init = update Synchronize init
        , view = view
        , subscriptions = subscriptions
        , update = update
        }


subscriptions : Model -> Sub Msg
subscriptions model =
    WebSocket.listen model.settings.townUrl TownServerMsg
