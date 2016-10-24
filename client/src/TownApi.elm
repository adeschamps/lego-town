module TownApi exposing (..)

import Json.Encode as Enc
import Json.Decode exposing (..)
import Json.Decode.Extra exposing (..)

-- INCOMING MESSAGES

type alias BuildingId = Int

type Msg
    = Initialize (List BuildingInfo)
    | SetLights BuildingId (List LightState)

type alias BuildingInfo =
    { buildingId : Int
    , name : String
    , lights : List LightState
    }

type alias LightState =
    { lightId : Int
    , isOn : Bool
    }

-- DECODERS

buildingId = int

msg : Decoder Msg
msg = ("type" := string) `andThen` subMsg

subMsg : String -> Decoder Msg
subMsg msgType =
    case msgType of
        "initialize" ->
            succeed Initialize
                |: ("buildings" := list buildingInfo)
        "setLights" ->
            succeed SetLights
                |: ("buildingId" := buildingId)
                |: ("lights" := list lightState)
        _ -> fail ("invalid message type: " ++ msgType)

buildingInfo : Decoder BuildingInfo
buildingInfo =
    succeed BuildingInfo
        |: ("buildingId" := int)
        |: ("name" := string)
        |: ("lights" := list lightState)

lightState : Decoder LightState
lightState =
    succeed LightState
        |: ("lightId" := int)
        |: ("isOn" := bool)

-- OUTGOING MESSAGES

init : Value
init =
    Enc.object
        [ ("type", Enc.string "init")
        ]
