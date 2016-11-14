module Town exposing (..)

import TownApi

import Color exposing (Color)
import Dict exposing (Dict)

-- MODEL

type alias Model =
    { buildings : Dict Int Building
    }

type alias Building =
    { name : String
    , lights : Dict Int Color
    }

genericBuilding : Building
genericBuilding =
    { name = "Generic Building"
    , lights = Dict.empty
    }

init : Model
init =
    { buildings = Dict.empty
    --  |> Dict.insert 0 genericBuilding
    }

-- UPDATE

type Msg
    = SetBuildings (List TownApi.BuildingInfo)

update : Msg -> Model -> Model
update msg model =
    case msg of
        SetBuildings buildingInfo ->
            let
                makeBuilding info = (info.buildingId, { name = info.name
                                                      , lights = Dict.empty
                                                      })
                buildings = buildingInfo
                          |> List.map makeBuilding
                          |> Dict.fromList
            in
                { model | buildings = buildings }
