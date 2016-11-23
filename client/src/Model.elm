module Model exposing (..)

-- LOCAL

import Settings
import SettingsPage
import Town
import TownApi
import TownPage


--EXTERNAL

import Json.Decode
import Json.Encode
import Material
import OutMessage
import WebSocket


-- MODEL


type alias Model =
    { town : Town.Model
    , settings : Settings.Model
    , townPage : TownPage.Model
    , settingsPage : SettingsPage.Model
    , errorMsg : String
    , mdl : Material.Model
    }


init : ( Model, Cmd Msg )
init =
    let
        model =
            { town = Town.init
            , settings = Settings.init
            , townPage = TownPage.init
            , settingsPage = SettingsPage.init
            , errorMsg = ""
            , mdl = Material.model
            }

        cmd =
            Cmd.none
    in
        ( model, cmd )



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



-- SUBSCRIPTIONS


subscriptions : Model -> Sub Msg
subscriptions model =
    WebSocket.listen model.settings.townUrl TownServerMsg
