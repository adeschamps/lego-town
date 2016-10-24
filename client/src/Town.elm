module Town exposing (..)

import Dict exposing (Dict)

type alias Town =
    { buildings : Dict Int Building
    }

type alias Building =
    { name : String
    , lights : Dict Int Light
    }

type alias Light =
    { lightState : LightState
    }

type LightState
    = On
    | Off


cafeCorner : Building
cafeCorner =
    { name = "Cafe Corner"
    , lights = Dict.empty
    }

greenGrocer : Building
greenGrocer =
    { name = "Green Grocer"
    , lights = Dict.empty
    }

init : Town
init =
    { buildings = Dict.empty
    |> Dict.insert 0 cafeCorner
    |> Dict.insert 1 greenGrocer
    }
