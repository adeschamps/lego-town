module Town exposing (..)

import Color exposing (Color)
import Dict exposing (Dict)

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
    |> Dict.insert 0 genericBuilding
    }
