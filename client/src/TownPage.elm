module TownPage exposing (Model, Msg, OutMsg(..), init, update, view)

import Color as StdColor exposing (hsl)
import Color.Convert exposing (colorToHex)

import Dict

import Html exposing (..)

import List.Extra as List

import Material
import Material.Button as Button
import Material.Card as Card
import Material.Color as Color
import Material.Elevation as Elevation
import Material.Icon as Icon
import Material.Options as Options

import Parts exposing (Index)

import Town
import TownApi


type alias Model =
    { expanded : Maybe Town.BuildingId
    , mdl : Material.Model
    }

init : Model
init =
    { expanded = Nothing
    , mdl = Material.model
    }

type Msg
    = Expand Town.BuildingId
    | Collapse
    | SetBuildingColor Town.BuildingId StdColor.Color
    | Mdl (Material.Msg Msg)


type OutMsg
    = Api TownApi.Type

update : Msg -> Model -> (Model, Cmd Msg, Maybe OutMsg)
update msg model =
    case msg of
        Expand buildingId ->
            let model = { model | expanded = Just buildingId }
            in (model, Cmd.none, Nothing)

        Collapse ->
            let model = { model | expanded = Nothing }
            in (model, Cmd.none, Nothing)

        SetBuildingColor id color ->
            let outMsg = Api <| TownApi.setBuilding id color
            in (model, Cmd.none, Just outMsg)

        Mdl msg' -> let (model, cmd) = Material.update msg' model
                    in (model, cmd, Nothing)


view : Town.Model -> Model -> Html Msg
view town model =
    Options.div []
        <| List.map (\(k, b) -> viewBuilding (k::[]) model b)
        <| Dict.toList town.buildings


viewBuilding : Index -> Model -> Town.Building -> Html Msg
viewBuilding index model building =
    let
        expanded = model.expanded == Just building.id
        expandButton = viewExpandButton (0::index) model.mdl expanded building.id
        advancedActions = colorPicker (1::index) model.mdl (SetBuildingColor building.id)
        offButton = Button.render Mdl (2::index) model.mdl [ Button.onClick <| SetBuildingColor building.id <| hsl 0 0 0 ] [ text "Off" ]
    in
        Card.view
            [ Elevation.e2
            , Options.css "backgroundColor" (colorToHex <| mainColor building)
--            , Color.background <| Color.color model.hue Color.S500
            ]
            [ Card.title [] [ Card.head [ Color.text Color.white ] [ text building.name ] ]
--            , Card.menu [] [ expandButton ]
            , Card.actions [ Color.background Color.white ] [ advancedActions , offButton ]
--                <| if expanded then advancedActions else []
            ]


viewExpandButton : Index -> Material.Model -> Bool -> Town.BuildingId -> Html Msg
viewExpandButton index mdl expanded buildingId =
    let
        (icon, onClick) =
            if expanded
            then ("expand_less", Collapse)
            else ("expand_more", Expand buildingId)
    in
        Button.render Mdl (0::index) mdl
            [ Button.icon
            , Button.ripple
            , Button.onClick onClick
            ]
            [ Icon.i icon
            ]


mainColor : Town.Building -> StdColor.Color
mainColor building =
    let
        unique = building.lights
               |> Dict.values
               |> List.uniqueBy (.color >> colorToHex)
    in
        case unique of
            [light] -> light.color
            _ -> hsl 0 0 0




-- UTIL
-- TODO: Move these to another module

-- Creates a list of buttons which emit a message when clicked.
colorPicker : Index -> Material.Model -> (StdColor.Color -> Msg) -> Html Msg
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
        rainbow 6 |> List.indexedMap makeButton |> Options.div []

-- Returns a list of colours evenly distributed around the hue circle.
rainbow : Int -> List StdColor.Color
rainbow count =
    let
        delta = 360 / (toFloat count) |> degrees
    in
        [0..count-1]
            |> List.map (\i -> (toFloat i) * delta)
            |> List.map (\hue -> hsl hue 1.0 0.5)
