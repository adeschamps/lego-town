module Settings exposing (..)

import Erl

type alias Settings =
    { townUrl : Erl.Url
    , arduinoUrl : Erl.Url
    }

init : Settings
init =
    { townUrl = Erl.parse "ws://127.0.0.1:1234"
    , arduinoUrl = Erl.parse "127.0.0.1:5000"
    }

type Msg
    = SetTownUrl Erl.Url
    | SetArduinoUrl Erl.Url

update : Msg -> Settings -> Settings
update msg model =
    case msg of
        SetTownUrl url -> { model | townUrl = url }
        SetArduinoUrl url -> { model | arduinoUrl = url }
