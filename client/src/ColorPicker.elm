module ColorPicker exposing (view)

-- LOCAL

import TownApi exposing (Color(..), colorToHex)


-- EXTERNAL

import Context exposing (withIndex)
import Html exposing (Html, div)
import Html.Attributes exposing (style)
import Html.Events exposing (onClick)
import Material
import Material.Button as Button
import Material.Icon as Icon
import Material.List as List
import Material.Options as Options


type alias Context msg =
    Context.Context Material.Model msg


colors : List Color
colors =
    [ WHITE, RED, ORANGE, YELLOW, GREEN, CYAN, BLUE, PURPLE, MAGENTA, OFF ]


view : (Color -> msg) -> Html msg
view onSelect =
    let
        baseButton width height color =
            div
                [ style
                    [ ( "backgroundColor", colorToHex color )
                    , ( "display", "inline-block" )
                    , ( "border", "1px solid black" )
                    , ( "width", width )
                    , ( "height", height )
                    , ( "padding", "0px" )
                    , ( "margin", "0px" )
                    ]
                , onClick <| onSelect color
                ]
                []

        button =
            baseButton "46px" "46px"

        wideButton =
            baseButton "94px" "46px"
    in
        div
            [ style
                [ ( "display", "block" )
                , ( "width", "192px" )
                , ( "font-size", "0" )
                ]
            ]
            [ button RED
            , button ORANGE
            , button YELLOW
            , button GREEN
            , button CYAN
            , button BLUE
            , button PURPLE
            , button MAGENTA
            , wideButton WHITE
            , wideButton OFF
            ]
