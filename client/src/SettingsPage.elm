module SettingsPage exposing (init, update, view, State, Msg, SettingConfig)

-- EXTERNAL

import Context exposing (child, with, withIndex)
import Html exposing (..)
import Material
import Material.Layout as Layout
import Material.Options as Options
import Material.Textfield as Textfield
import Maybe exposing (andThen)


type alias Context msg =
    Context.Context Material.Model msg



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
    Context msg
    -> State
    -> (Msg -> msg)
    -> List (SettingConfig msg)
    -> Html msg
view context state parent settings =
    let
        setting i config =
            viewSetting (child context i) state parent config
    in
        Options.div []
            [ Layout.title [] [ text "Settings" ]
            , Options.div [] <| List.indexedMap setting <| settings
            ]


viewSetting :
    Context msg
    -> State
    -> (Msg -> msg)
    -> SettingConfig msg
    -> Html msg
viewSetting context state parent config =
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
        (Textfield.render |> withIndex context 0)
            [ Textfield.floatingLabel
            , Textfield.label <| config.label
            , Textfield.value <| value
            , Textfield.onFocus <| parent <| BeginEditing config.id config.value
            , Textfield.onInput (parent << UpdateValue)
            , Textfield.onBlur <| config.onChange value
            ]
