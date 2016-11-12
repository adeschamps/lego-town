module TownPage exposing (..)

import Color as StdColor exposing (hsl)
import Color.Convert exposing (colorToHex)

import Dict

import Html exposing (..)
import Html.App
import Html.Attributes exposing (style)

import List.Extra as List exposing (takeWhile)

import Material
import Material.Button as Button
import Material.Card as Card
import Material.Color as Color
import Material.Elevation as Elevation
import Material.Grid as Grid exposing (grid, cell, size)
import Material.Icon as Icon
import Material.Layout as Layout
import Material.Options as Options

import OutMessage

import Parts exposing (Index)

import Building
import Town
import TownApi

type alias Model =
    { buildings : List Building.Model
    , mdl : Material.Model
    }

init : Town.Model -> Model
init town =
    { buildings = List.map Building.init <| Dict.values town.buildings
    , mdl = Material.model
    }

type alias BuildingId = Int

type Msg
    = SetBuilding Int StdColor.Color
    | SetLight Int Int StdColor.Color
    | UpdateBuilding BuildingId Building.Msg
    | Mdl (Material.Msg Msg)


type OutMsg
    = Api TownApi.Type

update : Msg -> Model -> (Model, Cmd Msg, Maybe OutMsg)
update msg model =
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

        UpdateBuilding id msg' ->
            case List.getAt id model.buildings of
                Nothing ->
                    (model, Cmd.none, Nothing)
                Just building ->
                    let
                        (newBuilding, cmd) = Building.update msg' building
                        newBuildings = case List.setAt id newBuilding model.buildings of
                                           Nothing -> model.buildings
                                           Just buildings -> buildings
                    in
                        ({model | buildings = newBuildings}, Cmd.none, Nothing)

        Mdl msg' -> let (model, cmd) = Material.update msg' model
                    in (model, cmd, Nothing)


view : Model -> Town.Model -> Html Msg
view model town =
    let
        viewBuilding id building = Html.App.map (UpdateBuilding id) <| Building.view building
    in
        Options.div []
            <| List.indexedMap viewBuilding model.buildings
