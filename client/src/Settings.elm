module Settings exposing (..)

import TownApi


type alias Model =
    { townUrl : String
    , arduinoUrl : String
    }


init : Model
init =
    { townUrl = "ws://192.168.1.136:1234"
    , arduinoUrl = ""
    }


type Msg
    = SetTownUrl String
    | SetArduinoUrl String


type OutMsg
    = Api TownApi.Type


update : Msg -> Model -> ( Model, Maybe OutMsg )
update msg model =
    case msg of
        SetTownUrl url ->
            ( { model | townUrl = url }, Nothing )

        SetArduinoUrl url ->
            let
                outMsg =
                    Api <| TownApi.setArduinoAddress url
            in
                ( { model | arduinoUrl = url }, Just outMsg )
