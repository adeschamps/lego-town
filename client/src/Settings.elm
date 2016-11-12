module Settings exposing (..)

import Erl

type alias Model =
    { townUrl : Erl.Url
    , arduinoUrl : Erl.Url
    }

init : Model
init =
    { townUrl = Erl.parse "ws://192.168.1.136:1234"
    , arduinoUrl = Erl.parse "127.0.0.1:5000"
    }

type Msg
    = SetTownUrl Erl.Url
    | SetArduinoUrl Erl.Url

update : Msg -> Model -> Model
update msg model =
    case msg of
        SetTownUrl url -> { model | townUrl = url }
        SetArduinoUrl url -> { model | arduinoUrl = url }
