module SettingsPage exposing (Model, Msg, OutMsg(..), init, update, view)

import Html exposing (..)

import List.Extra as List

import Material
import Material.Layout as Layout
import Material.Options as Options
import Material.Textfield as Textfield

import Maybe exposing (andThen)

import Parts exposing (Index)

import Settings

-- MODEL

type alias Mdl = Material.Model

type alias Model =
    { editing : Maybe SettingElement
    , inputValue : Maybe String
    , mdl : Material.Model
    }

init : Model
init =
    { editing = Nothing
    , inputValue = Nothing
    , mdl = Material.model
    }

type SettingElement
    = TownUrl
    | ArduinoUrl

type alias SettingInfo =
    { element : SettingElement
    , label : String
    , getValue : Settings.Model -> String
    , saveMessage : String -> Settings.Msg
    }

settingInfo : List SettingInfo
settingInfo =
    [ { element = TownUrl
      , label = "Town URL"
      , getValue = .townUrl
      , saveMessage = Settings.SetTownUrl
      }
    , { element = ArduinoUrl
      , label = "Arduino URL"
      , getValue = .arduinoUrl
      , saveMessage = Settings.SetArduinoUrl
      }
    ]


-- UPDATE

type Msg
    = BeginEditing SettingElement
    | UpdateValue String
    | EndEditing SettingElement
    | Mdl (Material.Msg Msg)

type OutMsg
    = SettingsMsg Settings.Msg

update : Msg -> Model -> (Model, Cmd Msg, Maybe OutMsg)
update msg model =
    case msg of
        BeginEditing element ->
            ({model | editing = Just element}, Cmd.none, Nothing)

        UpdateValue value ->
            ({model | inputValue = Just value}, Cmd.none, Nothing)

        EndEditing element ->
            let
                newModel = {model | editing = Nothing , inputValue = Nothing}
                outMsg = settingInfo
                       |> List.find (\i -> i.element == element)
                       |> Maybe.map2 (\value i -> i.saveMessage value |> SettingsMsg) model.inputValue
            in
                (newModel, Cmd.none, outMsg)

        Mdl msg' ->
            let
                (model, cmd) = Material.update msg' model
            in
                (model, cmd, Nothing)

-- VIEW

view : Settings.Model -> Model -> Html Msg
view settings model =
    let
        setting i info = viewSetting settings model (i::[]) info
    in
        Options.div []
            [ Layout.title [] [ text "Settings" ]
            , Options.div [] <| List.indexedMap setting <| settingInfo
            ]


viewSetting : Settings.Model -> Model -> Index -> SettingInfo -> Html Msg
viewSetting settings model index info =
    let
        editing = model.editing == Just info.element
        value = case (editing, model.inputValue) of
                    (True, Just value) -> value
                    (_, _) -> info.getValue settings
    in
        Textfield.render Mdl (0::index) model.mdl
            [ Textfield.floatingLabel
            , Textfield.label info.label
            , Textfield.value value
            , Textfield.onFocus <| BeginEditing info.element
            , Textfield.onInput UpdateValue
            , Textfield.onBlur <| EndEditing info.element
        ]
