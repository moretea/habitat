// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

import * as actionTypes from "../actions/index";
import initialState from "../initialState";

export default function router(state = initialState["router"], action) {
    switch (action.type) {
       case actionTypes.ROUTE_CHANGE:
            return state.set("route", action.payload).
                set("requestedRoute", null);

        case actionTypes.ROUTE_REQUESTED:
            return state.
                set("requestedRoute", action.payload);

        case actionTypes.SET_REDIRECT_ROUTE:
            return state.
                set("redirectRoute", action.payload);

        case actionTypes.RESET_REDIRECT_ROUTE:
            return state.
                set("redirectRoute", "");

        default:
            return state;
    }
}
