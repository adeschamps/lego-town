module Building exposing (..)

import Color as StdColor exposing (hsl)
import Color.Convert exposing (colorToHex)

import Html exposing (..)

import Material
import Material.Button as Button
import Material.Card as Card
import Material.Color as Color
import Material.Elevation as Elevation
import Material.Icon as Icon
import Material.Options as Options

import Parts exposing (Index)

import Town

type alias Model =
    { name : String
    , expanded : Bool
    , hue : Color.Hue
    , mdl : Material.Model
    }

type Msg
    = SetExpanded Bool
    | SetBuildingColor StdColor.Color
    | Mdl (Material.Msg Msg)

init : Town.Building -> Model
init building =
    { name = building.name
    , expanded = False
    , hue = buildingHue building.name
    , mdl = Material.model
    }

buildingHue : String -> Color.Hue
buildingHue name =
    case name of
        "Cafe Corner" ->
            Color.Brown
        "Green Grocer" ->
            Color.Green
        "Fire Brigade" ->
            Color.Red
        "Grand Emporium" ->
            Color.Amber
        -- More to fill in
        _ ->
            Color.Grey


update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
    case msg of
        SetExpanded expanded ->
            { model | expanded = expanded } ! []

        SetBuildingColor color ->
            model ! []

        Mdl msg' ->
            Material.update msg' model

view : Model -> Html Msg
view model =
    let
        index = []
        expandButton = viewExpandButton (0::index) model.mdl model.expanded
        advancedActions = colorPicker (1::index) model.mdl SetBuildingColor
    in
        Card.view
            [ Elevation.e2
            , Color.background <| Color.color model.hue Color.S500
            ]
            [ Card.title [] [ Card.head [] [ text model.name ] ]
            , Card.menu [] [ expandButton ]
            , Card.actions [ Color.background Color.white ]
                <| if model.expanded then advancedActions else []
            ]

viewExpandButton : Index -> Material.Model -> Bool -> Html Msg
viewExpandButton index mdl expanded =
    let
        icon = if expanded then "expand_less" else "expand_more"
    in
        Button.render Mdl (0::index) mdl
            [ Button.icon
            , Button.ripple
            , Button.onClick <| SetExpanded (not expanded)
            ]
            [ Icon.i icon
            ]







-- UTIL
-- TODO: Move these to another module

-- Creates a list of buttons which emit a message when clicked.
colorPicker : Index -> Material.Model -> (StdColor.Color -> Msg) -> List (Html Msg)
colorPicker index mdl onClick =
    let
        makeButton i color =
            Button.render Mdl (i::index) mdl
                [ Button.icon
                , Button.ripple
                , Color.text Color.white
                , Button.onClick <| onClick color
                , Options.css "backgroundColor" (colorToHex color)
                ]
            [ Icon.i "lightbulb_outline" ]
    in
        rainbow 6 |> List.indexedMap makeButton

-- Returns a list of colours evenly distributed around the hue circle.
rainbow : Int -> List StdColor.Color
rainbow count =
    let
        delta = 360 / (toFloat count) |> degrees
    in
        [0..count-1]
            |> List.map (\i -> (toFloat i) * delta)
            |> List.map (\hue -> hsl hue 1.0 0.5)
