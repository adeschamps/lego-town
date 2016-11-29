module View exposing (view)

-- LOCAL

import Model exposing (..)
import SettingsPage
import TownPage


-- EXTERNAL

import Context exposing (with, withIndex, child)
import Html exposing (..)
import Material
import Material.Color as Color
import Material.Layout as Layout
import Material.Scheme


type alias Context =
    Context.Context Material.Model Msg


view : Model -> Html Msg
view model =
    let
        context =
            Context.init Mdl model.mdl
    in
        (Layout.render |> with context)
            [ Layout.fixedHeader
            ]
            { header = [ header model ]
            , drawer = [ drawer (child context 0) model ]
            , tabs = ( [], [] )
            , main = body (child context 1) model
            }
            |> Material.Scheme.topWithScheme Color.Blue Color.LightGreen


header : Model -> Html Msg
header model =
    Layout.row []
        [ Layout.title [] [ text "LEGO Town" ]
        , Layout.spacer
        , Layout.navigation []
            [ syncButton model
            ]
        ]


syncButton : Model -> Html Msg
syncButton model =
    Layout.link
        [ Layout.onClick Synchronize ]
        [ text "Sync" ]


settingsInfo : Model -> List (SettingsPage.SettingConfig Msg)
settingsInfo model =
    [ { id = 0
      , label = "Server Url"
      , value = model.settings.serverUrl
      , onChange = SetServerUrl
      }
    , { id = 1
      , label = "Arduino Url"
      , value = model.settings.arduinoUrl
      , onChange = SetArduinoUrl
      }
    ]


drawer : Context -> Model -> Html Msg
drawer context model =
    settingsInfo model |> SettingsPage.view context model.settingsPage SettingsPageMsg


body : Context -> Model -> List (Html Msg)
body context model =
    [ Html.map UpdateTownPage <| TownPage.view model.town model.townPage
    , text model.errorMsg
    ]
