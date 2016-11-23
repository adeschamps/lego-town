module Tests exposing (..)

import Color exposing (Color)
import Expect
import Fuzz exposing (list, int, tuple, string)
import Json.Decode
import String
import Test exposing (..)
import TownApi


testDecode : String -> Json.Decode.Decoder t -> String -> t -> Test
testDecode name decoder json result =
    test name <| \() -> Expect.equal (Json.Decode.decodeString decoder json) (Ok result)


all : Test
all =
    describe "Town API Decoders"
        [ testDecode "State"
            TownApi.msg
            """{"type":"state","arduinoAddress":"127.0.0.1:12345","buildings":[]}"""
          <|
            TownApi.State "127.0.0.1:12345" []
        , testDecode "SetLights"
            TownApi.msg
            """{"type":"setLights", "buildingId":1, "lights":[{"lightId":3, "color":"#ff0000"}]}"""
          <|
            TownApi.SetLights 1 [ { lightId = 3, color = Color.rgb 255 0 0 } ]
        ]
