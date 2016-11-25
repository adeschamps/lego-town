module Util exposing (colorPicker)

import Color exposing (Color)
import Color.Convert exposing (colorToHex)
import Context exposing (withIndex)
import Html exposing (Html)
import Material
import Material.Button as Button
import Material.Color
import Material.Icon as Icon
import Material.Options as Options


type alias Context msg =
    Context.Context Material.Model msg


{-| Creates a list of buttons which emit a message when clicked.
-}
colorPicker :
    Context msg
    -> (Color -> msg)
    -> Html msg
colorPicker context onClick =
    let
        makeButton i color =
            (Button.render |> withIndex context i)
                [ Button.icon
                , Button.ripple
                , Material.Color.text Material.Color.white
                , Button.onClick <| onClick color
                , Options.css "backgroundColor" (colorToHex color)
                ]
                [ Icon.i "lightbulb_outline" ]
    in
        rainbow 6 |> List.indexedMap makeButton |> Options.div []


{-| Returns a list of colours evenly distributed around the hue circle.
-}
rainbow : Int -> List Color
rainbow count =
    let
        delta =
            360 / (toFloat count) |> degrees
    in
        List.range 0 (count - 1)
            |> List.map (\i -> (toFloat i) * delta)
            |> List.map (\hue -> Color.hsl hue 1.0 0.5)
