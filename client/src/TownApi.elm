module TownApi exposing (..)

import Color exposing (Color)
import Color.Convert exposing (..)

import Json.Encode as Enc
import Json.Decode exposing (..)
import Json.Decode.Extra exposing (..)

import Result exposing (fromMaybe)

-- INCOMING MESSAGES

type alias BuildingId = Int

type Msg
    = State String (List BuildingInfo)
    | SetLights BuildingId (List LightState)

type alias BuildingInfo =
    { buildingId : Int
    , name : String
    , lights : List LightState
    }

type alias LightState =
    { lightId : Int
    , color : Color
    }

-- DECODERS

buildingId : Decoder BuildingId
buildingId = int

msg : Decoder Msg
msg = ("type" := string) `andThen` subMsg

subMsg : String -> Decoder Msg
subMsg msgType =
    case msgType of
        "state" ->
            succeed State
                |: ("arduinoAddress" := string)
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
        |: ("color" := color)

color : Decoder Color
color =
    let
        decodeColor c = hexToColor c |>  fromMaybe ("Invalid color: " ++ c)
    in
        customDecoder string decodeColor


-- OUTGOING MESSAGES

-- Not sure if I like this name
-- maybe something like MessageType?
type alias Type = Value

getState : Value
getState =
    Enc.object
        [ ("type", Enc.string "getState")
        ]

setBuilding : Int -> Color -> Value
setBuilding buildingId color =
    Enc.object
        [ ("type",       Enc.string "setBuilding")
        , ("buildingId", Enc.int buildingId)
        , ("color",      encColor color)]

setLight : Int -> Int -> Color -> Value
setLight buildingId lightId color =
    Enc.object
        [ ("type",       Enc.string "setLight")
        , ("buildingId", Enc.int buildingId)
        , ("lightId",    Enc.int lightId)
        , ("color",      encColor color)
        ]

setArduinoAddress : String -> Value
setArduinoAddress address =
    Enc.object
        [ ("type",    Enc.string "setArduinoAddress")
        , ("address", Enc.string address)
        ]

encColor : Color -> Value
encColor = Enc.string << colorToHex
