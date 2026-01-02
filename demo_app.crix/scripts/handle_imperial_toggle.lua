-- handle_imperial_toggle.lua
--
-- Updates unit labels when the imperial checkbox is toggled.
-- Does NOT perform any calculations.
--
-- Inputs (from Store):
--   settings.imperial - If true, use gallons; if false, use liters
--
-- Outputs (to Store):
--   labels.fuel_unit   - Label text for fuel input
--   labels.result_unit - Label text for result output

app.log("Handling imperial toggle...")

local imperial = app.get("settings.imperial")

if imperial then
    app.set("labels.fuel_unit", "Current Fuel (gallons)")
    app.set("labels.result_unit", "E85 to Add (gallons)")
    app.log("Switched to imperial units (gallons)")
else
    app.set("labels.fuel_unit", "Current Fuel (liters)")
    app.set("labels.result_unit", "E85 to Add (liters)")
    app.log("Switched to metric units (liters)")
end
