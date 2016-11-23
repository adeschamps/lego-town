module App exposing (main)

-- EXTERNAL MODULES

import Html


-- LOCAL MODULES

import Model
import View


main : Program Never Model.Model Model.Msg
main =
    Html.program
        { init = Model.init
        , view = View.view
        , subscriptions = Model.subscriptions
        , update = Model.update
        }
