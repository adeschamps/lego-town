module Town exposing (..)

import TownApi

import Color exposing (Color)

import Dict exposing (Dict)
import Dict.Extra as Dict

-- MODEL

type alias BuildingId = Int
type alias LightId = Int

type alias Model =
    { buildings : Dict BuildingId Building
    }

type alias Building =
    { id : BuildingId
    , name : String
    , lights : Dict LightId Light
    }

type alias Light =
    { id : LightId
    , color : Color
    }

genericBuilding : Building
genericBuilding =
    { id = -1
    , name = "Generic Building"
    , lights = Dict.empty
    }

init : Model
init =
    { buildings = Dict.empty
    |> Dict.insert 0 genericBuilding
    }

-- UPDATE

type Msg
    = SetBuildings (List TownApi.BuildingInfo)

update : Msg -> Model -> Model
update msg model =
    case msg of
        SetBuildings buildingInfo ->
            let
                buildings = buildingInfo
                          |> List.map infoToBuilding
                          |> Dict.fromListBy .id
            in
                { model | buildings = buildings }

infoToBuilding : TownApi.BuildingInfo -> Building
infoToBuilding info =
    { id = info.buildingId
    , name = info.name
    , lights = info.lights |> List.map stateToLight |> Dict.fromListBy .id
    }

stateToLight : TownApi.LightState -> Light
stateToLight state =
    { id = state.lightId
    , color = state.color
    }
