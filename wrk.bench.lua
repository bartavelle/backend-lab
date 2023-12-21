lh = require("luahash")

token = nil
phase = 0
nextbody = nil

wrk.method = "POST"
wrk.headers["Content-Type"] = "application/json"

request = function()
  if phase == 0 then
    wrk.body = "{\"username\":\"bob\"}"
    return wrk.format("POST", "/preauth")
  elseif phase == 1 then
    wrk.body = nextbody
    return wrk.format("POST", "/auth")
  else
    wrk.body = "{\"clicks\": 1, \"pages\": 2, \"speed\": 3}"
    return wrk.format("POST", "/perf")
  end
end

response = function(status, headers, body)
  if phase == 0 then
    nextbody = lh.hash_username("bob", body)
    phase = 1
  elseif phase == 1 then
    cookie = headers["set-cookie"]
    wrk.headers["cookie"] = cookie
    phase = 2
  end
end