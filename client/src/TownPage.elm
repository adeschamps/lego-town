module TownPage exposing (..)

import Color as StdColor exposing (hsl)
import Color.Convert exposing (colorToHex)

import Dict

import Html exposing (..)
import Html.Attributes exposing (style)

import List.Extra exposing (takeWhile)

import Material
import Material.Button as Button
import Material.Card as Card
import Material.Color as Color
import Material.Elevation as Elevation
import Material.Grid as Grid exposing (grid, cell, size)
import Material.Icon as Icon
import Material.Layout as Layout
import Material.Options as Options

import Parts exposing (Index)

import Town exposing (Town, Building)
import TownApi

type alias Model =
    { expandedBuilding : Maybe Int
    , mdl : Material.Model
    }

init : Town -> Model
init town =
    { expandedBuilding = Nothing
    , mdl = Material.model
    }

type Msg
    = SetBuilding Int StdColor.Color
    | SetLight Int Int StdColor.Color
    | Mdl (Material.Msg Msg)


type OutMsg
    = Api TownApi.Type

update : state -> Msg -> Model -> (Model, Cmd Msg, Maybe OutMsg)
update state msg model =
    case msg of
        SetBuilding buildingId color ->
            ( model
            , Cmd.none
            , Just <| Api <| TownApi.setBuilding buildingId color
            )

        SetLight buildingId lightId color ->
            ( model
            , Cmd.none
            , Just <| Api <| TownApi.setLight buildingId lightId color
            )

        Mdl msg' -> let (model, cmd) = Material.update msg' model
                    in (model, cmd, Nothing)


view : Model -> Town -> Html Msg
view model town =
    let
        building key = viewBuilding (key::[]) model town key
    in
        Options.div []
            <| List.map building (Dict.keys town.buildings)


viewBuilding : Index -> Model -> Town -> Int -> Html Msg
viewBuilding index model town buildingId =
    case Dict.get buildingId town.buildings of
        Nothing -> div [] []
        Just building ->
            let
                head = Card.head []
                       [ text building.name
                       ]
                style = buildingStyle building.name
                buildingColor = Color.color style.hue Color.S500
                expandButton = viewExpandButton (index) model.mdl
            in
                Card.view
                    [ Elevation.e2
                    , Color.background buildingColor
                    ]
                    [ Card.title [] [ head ]
                    , Card.menu [] [ expandButton ]
                    , Card.actions [ Color.background Color.white ]
                        <| colorPicker (1::index) model.mdl (SetBuilding buildingId)
                    ]

viewExpandButton : Index -> Material.Model -> Html Msg
viewExpandButton index mdl =
    let
        icon = "expand_more"
    in
        Button.render Mdl (0::index) mdl
            [ Button.icon
            , Button.ripple
            ]
            [ Icon.i icon
            ]

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

type alias BuildingStyle =
    { hue : Color.Hue
    }

buildingStyle : String -> BuildingStyle
buildingStyle name =
    case name of
        "Cafe Corner" ->
            { hue = Color.Brown
            }

        "Green Grocer" ->
            { hue = Color.Green
            }

        "Fire Brigade" ->
            { hue = Color.Red
            }

        "Grand Emporium" ->
            { hue = Color.Amber
            }

        -- More to fill in

        _ ->
            { hue = Color.Grey
            }
