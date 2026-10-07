## Signs in on the account website (web/ in the repository) and returns the short-lived
## token the game server accepts. The password is sent to the website only.
extends Node

const TIMEOUT := 20.0


## {"token": ...} or {"error": "why"}.
func login(website: String, email: String, password: String) -> Dictionary:
	var request := HTTPRequest.new()
	request.timeout = TIMEOUT
	add_child(request)
	var url := website.strip_edges().trim_suffix("/") + "/api/game/login"
	var body := JSON.stringify({"email": email.strip_edges(), "password": password})
	var error := request.request(url, ["Content-Type: application/json"], HTTPClient.METHOD_POST, body)
	if error != OK:
		request.queue_free()
		return {"error": "Can't reach the account website at %s." % website}
	var result: Array = await request.request_completed
	request.queue_free()
	if result[0] != HTTPRequest.RESULT_SUCCESS:
		return {"error": "Can't reach the account website at %s." % website}
	var answer = JSON.parse_string((result[3] as PackedByteArray).get_string_from_utf8())
	if typeof(answer) != TYPE_DICTIONARY:
		return {"error": "The account website gave a strange answer (HTTP %d)." % result[1]}
	if result[1] == 200 and answer.has("token"):
		return {"token": String(answer["token"])}
	return {"error": String(answer.get("error", "Signing in failed (HTTP %d)." % result[1]))}
