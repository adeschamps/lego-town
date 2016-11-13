module SettingsPage exposing (Model, Msg, OutMsg(..), init, update, view)

import Html exposing (..)

import Material
import Material.Layout as Layout
import Material.Options as Options
import Material.Textfield as Textfield

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
    , value : String
    , saveMessage : String -> Settings.Msg
    }

settingInfo : Settings.Model -> List SettingInfo
settingInfo settings =
    [ { element = TownUrl
      , label = "Town URL"
      , value = settings.townUrl
      , saveMessage = Settings.SetTownUrl
      }
    , { element = ArduinoUrl
      , label = "Arduino URL"
      , value = settings.arduinoUrl
      , saveMessage = Settings.SetArduinoUrl
      }
    ]


-- UPDATE

type Msg
    = BeginEditing SettingElement
    | UpdateValue String
    | EndEditing SettingInfo
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

        EndEditing info ->
            let
                newModel = {model | editing = Nothing , inputValue = Nothing}
                setValue = Maybe.map (SettingsMsg << info.saveMessage) model.inputValue
            in
                (newModel, Cmd.none, setValue)

        Mdl msg' ->
            let
                (model, cmd) = Material.update msg' model
            in
                (model, cmd, Nothing)

-- VIEW

view : Settings.Model -> Model -> Html Msg
view settings model =
    let
        setting i info = viewSetting model (i::[]) info
    in
        Options.div []
            [ Layout.title [] [ text "Settings" ]
            , Options.div [] <| List.indexedMap setting <| settingInfo settings
            ]


viewSetting : Model -> Index -> SettingInfo -> Html Msg
viewSetting model index info =
    let
        editing = model.editing == Just info.element
        value = case (editing, model.inputValue) of
                    (True, Just value) -> value
                    (_, _) -> info.value
    in
        Textfield.render Mdl (0::index) model.mdl
            [ Textfield.floatingLabel
            , Textfield.label info.label
            , Textfield.value value
            , Textfield.onFocus <| BeginEditing info.element
            , Textfield.onInput UpdateValue
            , Textfield.onBlur <| EndEditing info
        ]
