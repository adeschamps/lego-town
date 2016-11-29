module TownApi exposing (..)

import Json.Encode as Enc
import Json.Decode exposing (..)
import Json.Decode.Extra exposing (..)


-- INCOMING MESSAGES


type alias BuildingId =
    Int


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


type Color
    = OFF
    | WHITE
    | RED
    | ORANGE
    | YELLOW
    | GREEN
    | CYAN
    | BLUE
    | PURPLE
    | MAGENTA



-- DECODERS


buildingId : Decoder BuildingId
buildingId =
    int


msg : Decoder Msg
msg =
    (field "type" string) |> andThen subMsg


subMsg : String -> Decoder Msg
subMsg msgType =
    case msgType of
        "state" ->
            succeed State
                |: (field "arduinoAddress" string)
                |: (field "buildings" (list buildingInfo))

        "setLights" ->
            succeed SetLights
                |: (field "buildingId" buildingId)
                |: (field "lights" (list lightState))

        _ ->
            fail ("invalid message type: " ++ msgType)


buildingInfo : Decoder BuildingInfo
buildingInfo =
    succeed BuildingInfo
        |: (field "buildingId" int)
        |: (field "name" string)
        |: (field "lights" (list lightState))


lightState : Decoder LightState
lightState =
    succeed LightState
        |: (field "lightId" int)
        |: (field "color" color)


color : Decoder Color
color =
    let
        decodeColor c =
            case c of
                "OFF" ->
                    succeed OFF

                "WHITE" ->
                    succeed WHITE

                "RED" ->
                    succeed RED

                "ORANGE" ->
                    succeed ORANGE

                "YELLOW" ->
                    succeed YELLOW

                "GREEN" ->
                    succeed GREEN

                "CYAN" ->
                    succeed CYAN

                "BLUE" ->
                    succeed BLUE

                "PURPLE" ->
                    succeed PURPLE

                "MAGENTA" ->
                    succeed MAGENTA

                _ ->
                    fail ("Invalid color: " ++ c)
    in
        string |> andThen decodeColor



-- OUTGOING MESSAGES
-- Not sure if I like this name
-- maybe something like MessageType?


type alias Type =
    Value


getState : Value
getState =
    Enc.object
        [ ( "type", Enc.string "getState" )
        ]


setBuilding : Int -> Color -> Value
setBuilding buildingId color =
    Enc.object
        [ ( "type", Enc.string "setBuilding" )
        , ( "buildingId", Enc.int buildingId )
        , ( "color", encColor color )
        ]


setLight : Int -> Int -> Color -> Value
setLight buildingId lightId color =
    Enc.object
        [ ( "type", Enc.string "setLight" )
        , ( "buildingId", Enc.int buildingId )
        , ( "lightId", Enc.int lightId )
        , ( "color", encColor color )
        ]


setArduinoAddress : String -> Value
setArduinoAddress address =
    Enc.object
        [ ( "type", Enc.string "setArduinoAddress" )
        , ( "address", Enc.string address )
        ]


encColor : Color -> Value
encColor color =
    let
        colorName =
            case color of
                OFF ->
                    "OFF"

                WHITE ->
                    "WHITE"

                RED ->
                    "RED"

                ORANGE ->
                    "ORANGE"

                YELLOW ->
                    "YELLOW"

                GREEN ->
                    "GREEN"

                CYAN ->
                    "CYAN"

                BLUE ->
                    "BLUE"

                PURPLE ->
                    "PURPLE"

                MAGENTA ->
                    "MAGENTA"
    in
        Enc.string colorName


colorToHex : Color -> String
colorToHex color =
    case color of
        OFF ->
            "#000000"

        WHITE ->
            "#ffffff"

        RED ->
            "#ff0000"

        ORANGE ->
            "#ff8800"

        YELLOW ->
            "#ffff00"

        GREEN ->
            "#00ff00"

        CYAN ->
            "#00ffff"

        BLUE ->
            "#0000ff"

        PURPLE ->
            "#8800ff"

        MAGENTA ->
            "#ff00ff"
