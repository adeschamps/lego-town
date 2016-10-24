module TownPage exposing (..)

import Dict

import Html exposing (..)

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
    = Mdl (Material.Msg Msg)


type OutMsg
    = NoOp

update : state -> Msg -> Model -> (Model, Cmd Msg, Maybe OutMsg)
update state msg model =
    case msg of
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
                        [ Button.render Mdl (0::index) model.mdl [ Button.icon, Button.ripple ] [ Icon.i "lightbulb_outline" ]
                        ]
                    ]


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
