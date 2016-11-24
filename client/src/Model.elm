module Model exposing (..)

-- LOCAL

import Town
import TownApi
import TownPage
import SettingsPage


--EXTERNAL

import Json.Decode
import Json.Encode
import Material
import OutMessage
import WebSocket


-- MODEL


type alias Model =
    { town : Town.Model
    , settings : Settings
    , townPage : TownPage.Model
    , settingsPage : SettingsPage.State
    , errorMsg : String
    , mdl : Material.Model
    }


type alias Settings =
    { serverUrl : String
    , arduinoUrl : String
    }


init : ( Model, Cmd Msg )
init =
    let
        settings =
            { serverUrl = "ws://192.168.1.136:1234"
            , arduinoUrl = ""
            }

        model =
            { town = Town.init
            , settings = settings
            , townPage = TownPage.init
            , settingsPage = SettingsPage.init
            , errorMsg = ""
            , mdl = Material.model
            }

        cmd =
            townServerCmd model TownApi.getState
    in
        ( model, cmd )



-- UPDATE


type Msg
    = Synchronize
      -- Settings
    | SetServerUrl String
    | SetArduinoUrl String
    | SettingsPageMsg SettingsPage.Msg
      -- Town
    | UpdateTownPage TownPage.Msg
    | TownServerMsg String
      -- UI
    | Mdl (Material.Msg Msg)


update : Msg -> Model -> ( Model, Cmd Msg )
update msg ({ settings } as model) =
    case msg of
        Synchronize ->
            model ! [ townServerCmd model TownApi.getState ]

        SetServerUrl url ->
            let
                newModel =
                    { model | settings = { settings | serverUrl = url } }
            in
                newModel ! []

        SetArduinoUrl url ->
            let
                newModel =
                    { model | settings = { settings | arduinoUrl = url } }

                cmd =
                    TownApi.setArduinoAddress url |> townServerCmd model
            in
                ( newModel, cmd )

        SettingsPageMsg pageMsg ->
            let
                settingsPage =
                    model.settingsPage

                newSettingsPage =
                    SettingsPage.update pageMsg settingsPage

                newModel =
                    { model | settingsPage = newSettingsPage }
            in
                newModel ! []

        UpdateTownPage msg_ ->
            TownPage.update msg_ model.townPage
                |> OutMessage.mapComponent
                    (\newTownPage -> { model | townPage = newTownPage })
                |> OutMessage.mapCmd UpdateTownPage
                |> OutMessage.evaluateMaybe handleTownMsg Cmd.none

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
    WebSocket.send model.settings.serverUrl (Json.Encode.encode 0 value)



-- SUBSCRIPTIONS


subscriptions : Model -> Sub Msg
subscriptions model =
    WebSocket.listen model.settings.serverUrl TownServerMsg
