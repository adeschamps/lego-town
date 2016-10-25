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

-- Not sure if I like this name
-- maybe something like MessageType?
type alias Type = Value

init : Value
init =
    Enc.object
        [ ("type", Enc.string "init")
        ]

setBuilding : Int -> Bool -> Value
setBuilding buildingId isOn =
    Enc.object
        [ ("type",       Enc.string "setBuilding")
        , ("buildingId", Enc.int buildingId)
        , ("isOn",       Enc.bool isOn)]

setLight : Int -> Int -> Bool -> Value
setLight buildingId lightId isOn =
    Enc.object
        [ ("type",       Enc.string "setLight")
        , ("buildingId", Enc.int buildingId)
        , ("lightId",    Enc.int lightId)
        , ("isOn",       Enc.bool isOn)
        ]
