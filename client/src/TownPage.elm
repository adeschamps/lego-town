module TownPage exposing (..)

import Color as StdColor exposing (hsl)

import Dict

import Html exposing (..)

import List.Extra exposing (takeWhile)

import Material
import Material.Button as Button
import Material.Card as Card
import Material.Color as Color
import Material.Grid as Grid exposing (grid, cell, size)
import Material.Icon as Icon
import Material.Layout as Layout
import Material.Options as Options

import Parts exposing (Index)

import Town exposing (Town, Building)
import TownApi

type alias Model =
    {
        mdl : Material.Model
    }

init : Town -> Model
init town =
    {
        mdl = Material.model
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
    Options.div []
        <| List.map (\k -> viewBuilding (k::[]) model town k) (Dict.keys town.buildings)


viewBuilding : Index -> Model -> Town -> Int -> Html Msg
viewBuilding index model town buildingId =
    case Dict.get buildingId town.buildings of
        Nothing -> div [] []
        Just building ->
            let
                style = buildingStyle building.name
            in
                Card.view
                    [ Color.background (Color.color style.hue Color.S500)
                    ]
                    [ Card.title [] [ Card.head [ Color.text Color.white ] [ text building.name ] ]
                    , Card.text [ Card.expand ] [] -- filler
                    , Card.actions []
                        <| List.indexedMap (\i color -> viewColorButton (i::index) model color (SetBuilding buildingId)) lightColors
            {-
                          [ Button.render Mdl (0::index) model.mdl
                                [ Button.icon, Button.ripple
                                , Color.text Color.white
                                , Color.background Color.white
                                , Button.onClick <| SetBuilding buildingId StdColor.white
                                ]
                                [ Icon.i "lightbulb_outline" ]
                          ]
             -}
                    ]

viewColorButton : Index -> Model -> StdColor.Color -> (StdColor.Color -> Msg) -> Html Msg
viewColorButton index model color onClick =
    Button.render Mdl (0::index) model.mdl
        [ Button.icon
        , Button.ripple
--        , Color.text color
        , Color.background Color.white
        , Button.onClick <| onClick color
        ]
        [ Icon.i "lightbulb_outline" ]

lightColors : List StdColor.Color
lightColors =
    [1..11]
        |> List.map (\d -> d * 30 |> degrees)
        |> takeWhile (\h -> h < 360)
        |> List.map (\h -> hsl h 1.0 0.5)

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
