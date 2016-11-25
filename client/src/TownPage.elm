module TownPage exposing (Model, Msg, OutMsg(..), init, update, view)

-- LOCAL

import Town
import TownApi


-- EXTERNAL

import Color as StdColor exposing (hsl)
import Color.Convert exposing (colorToHex)
import Context exposing (Context, child, with, withIndex)
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
import Util exposing (colorPicker)


type alias Context =
    Context.Context Material.Model Msg


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


update : Msg -> Model -> ( Model, Cmd Msg, Maybe OutMsg )
update msg model =
    case msg of
        Expand buildingId ->
            let
                newModel =
                    { model | expanded = Just buildingId }
            in
                ( newModel, Cmd.none, Nothing )

        Collapse ->
            let
                newModel =
                    { model | expanded = Nothing }
            in
                ( newModel, Cmd.none, Nothing )

        SetBuildingColor id color ->
            let
                outMsg =
                    Api <| TownApi.setBuilding id color
            in
                ( model, Cmd.none, Just outMsg )

        Mdl mdlMsg ->
            let
                ( newModel, cmd ) =
                    Material.update mdlMsg model
            in
                ( newModel, cmd, Nothing )


view : Town.Model -> Model -> Html Msg
view town model =
    let
        context =
            Context.init Mdl model.mdl
    in
        Options.div [] <|
            List.map (\( k, b ) -> (viewBuilding (child context k) model b)) <|
                Dict.toList town.buildings


viewBuilding : Context -> Model -> Town.Building -> Html Msg
viewBuilding context model building =
    let
        expanded =
            model.expanded == Just building.id

        expandButton =
            viewExpandButton (child context 0) expanded building.id

        advancedActions =
            colorPicker (child context 1) (SetBuildingColor building.id)

        offButton =
            (Button.render |> withIndex context 2)
                [ Button.onClick <| SetBuildingColor building.id <| hsl 0 0 0 ]
                [ text "Off" ]
    in
        Card.view
            [ Elevation.e2
            , Options.css "backgroundColor" (colorToHex <| mainColor building)
              --            , Color.background <| Color.color model.hue Color.S500
            ]
            [ Card.title [] [ Card.head [ Color.text Color.white ] [ text building.name ] ]
              --            , Card.menu [] [ expandButton ]
            , Card.actions [ Color.background Color.white ] [ advancedActions, offButton ]
              --                <| if expanded then advancedActions else []
            ]


viewExpandButton : Context -> Bool -> Town.BuildingId -> Html Msg
viewExpandButton context expanded buildingId =
    let
        ( icon, onClick ) =
            if expanded then
                ( "expand_less", Collapse )
            else
                ( "expand_more", Expand buildingId )
    in
        (Button.render |> withIndex context 0)
            [ Button.icon
            , Button.ripple
            , Button.onClick onClick
            ]
            [ Icon.i icon
            ]


mainColor : Town.Building -> StdColor.Color
mainColor building =
    let
        unique =
            building.lights
                |> Dict.values
                |> List.uniqueBy (.color >> colorToHex)
    in
        case unique of
            [ light ] ->
                light.color

            _ ->
                hsl 0 0 0
