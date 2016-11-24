module View exposing (..)

-- LOCAL

import Model exposing (..)
import SettingsPage
import TownPage


-- EXTERNAL

import Html exposing (..)
import Material.Color as Color
import Material.Layout as Layout
import Material.Scheme


view : Model -> Html Msg
view model =
    Layout.render Mdl
        model.mdl
        [ Layout.fixedHeader
        ]
        { header = [ header model ]
        , drawer = [ drawer model ]
        , tabs = ( [], [] )
        , main = body model
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


drawer : Model -> Html Msg
drawer model =
    settingsInfo model |> SettingsPage.view Mdl [] model.mdl model.settingsPage SettingsPageMsg


body : Model -> List (Html Msg)
body model =
    [ Html.map UpdateTownPage <| TownPage.view model.town model.townPage
    , text model.errorMsg
    ]
