module SettingsPage exposing (init, update, view, State, Msg, SettingConfig)

-- EXTERNAL

import Html exposing (..)
import Material
import Material.Layout as Layout
import Material.Options as Options
import Material.Textfield as Textfield
import Maybe exposing (andThen)
import Parts


type alias Index =
    Parts.Index (List Int)



-- MODEL


type alias State =
    { editing :
        Maybe
            { id : Int
            , value : String
            }
    }


init : State
init =
    { editing = Nothing
    }


type alias SettingConfig msg =
    { id : Int
    , label : String
    , value : String
    , onChange : String -> msg
    }



-- UPDATE


type Msg
    = BeginEditing Int String
    | UpdateValue String
    | EndEditing


update : Msg -> State -> State
update msg state =
    case msg of
        BeginEditing id value ->
            { state | editing = Just { id = id, value = value } }

        UpdateValue value ->
            let
                newEditing =
                    case state.editing of
                        Just editing ->
                            Just { editing | value = value }

                        Nothing ->
                            Nothing
            in
                { state | editing = newEditing }

        EndEditing ->
            { state | editing = Nothing }



-- VIEW


view :
    (Parts.Msg Material.Model msg -> msg)
    -> Index
    -> Material.Model
    -> State
    -> (Msg -> msg)
    -> List (SettingConfig msg)
    -> Html msg
view wrap index mdl state parent settings =
    let
        viewSetting i config =
            setting wrap (i :: index) mdl state parent config
    in
        Options.div []
            [ Layout.title [] [ text "Settings" ]
            , Options.div [] <| List.indexedMap viewSetting <| settings
            ]


setting :
    (Parts.Msg Material.Model msg -> msg)
    -> Index
    -> Material.Model
    -> State
    -> (Msg -> msg)
    -> SettingConfig msg
    -> Html msg
setting wrap index mdl state parent config =
    let
        id =
            config.id

        ( editing, value ) =
            let
                default =
                    ( False, config.value )
            in
                case state.editing of
                    Just editing ->
                        if editing.id == id then
                            ( True, editing.value )
                        else
                            default

                    Nothing ->
                        default
    in
        Textfield.render wrap
            (0 :: index)
            mdl
            [ Textfield.floatingLabel
            , Textfield.label <| config.label
            , Textfield.value <| value
            , Textfield.onFocus <| parent <| BeginEditing config.id config.value
            , Textfield.onInput (parent << UpdateValue)
            , Textfield.onBlur <| config.onChange value
            ]
