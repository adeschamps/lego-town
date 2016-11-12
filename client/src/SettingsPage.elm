module SettingsPage exposing (..)

import Html exposing (..)

import Erl

import Material
import Material.Layout as Layout
import Material.Options as Options
import Material.Textfield as Textfield
import Material.Toggles as Toggles

import Parts exposing (Index)

import String exposing (..)

import Settings

-- MODEL

type alias Mdl = Material.Model

type alias Model =
    { townUrl : String
    , arduinoUrl : String
    , mdl : Material.Model
    }

init : Settings.Model -> Model
init settings =
    { townUrl = Erl.toString settings.townUrl
    , arduinoUrl = Erl.toString settings.arduinoUrl
    , mdl = Material.model
    }

-- UPDATE

type Msg
    = UpdateTown String
    | CommitTown
    | UpdateArduino String
    | CommitArduino
    | Mdl (Material.Msg Msg)

type OutMsg
    = SetTownUrl Erl.Url
    | SetArduinoUrl Erl.Url

update : Msg -> Model -> (Model, Cmd Msg, Maybe OutMsg)
update msg model =
    case msg of
        UpdateTown url ->
            ({ model | townUrl = url }, Cmd.none, Nothing)

        CommitTown ->
            (model, Cmd.none, Just <| SetTownUrl <| Erl.parse model.townUrl)

        UpdateArduino url ->
            ({ model | arduinoUrl = url }, Cmd.none, Nothing)

        CommitArduino ->
            (model, Cmd.none, Just <| SetArduinoUrl <| Erl.parse model.arduinoUrl)

        Mdl msg' ->
            let
                (model, cmd) = Material.update msg' model
            in
                (model, cmd, Nothing)

-- VIEW

view : Model -> Settings.Model -> Html Msg
view model settings =
    Options.div []
        [ Layout.title [] [ text "Settings" ]
        , viewSettings [] model settings
        ]

type alias SettingInfo =
    { label : String
    , value : String
    , update : String -> Msg
    , commit : Msg
    }

settingInfo : Model -> Settings.Model -> List SettingInfo
settingInfo model settings =
    [ { label = "Server: " ++ Erl.toString settings.townUrl
      , value = model.townUrl
      , update = UpdateTown
      , commit = CommitTown
      }
    , { label = "Arduino: " ++ Erl.toString settings.arduinoUrl
      , value = model.arduinoUrl
      , update = UpdateArduino
      , commit = CommitArduino
      }
    ]

viewSettings : Index -> Model -> Settings.Model -> Html Msg
viewSettings index model settings =
    let
        viewSetting i info =
            Textfield.render Mdl (i::index) model.mdl
                [ Textfield.floatingLabel
                , Textfield.label   info.label
                , Textfield.value   info.value
                , Textfield.onInput info.update
                , Textfield.onBlur  info.commit
                ]
    in
        Options.div [] <|
            List.indexedMap viewSetting <| settingInfo model settings
