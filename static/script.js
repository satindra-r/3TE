import init, {
	setHook,
	render,
	handleKeyDown,
	handleMouseClick,
	handleDataIn,
	beginConnection,
	onDatabaseUpdate,
} from "../pkg/LearningWASM.js";

import {createClient} from '@supabase/supabase-js';

const supabaseUrl = 'https://rnjrfxqopylbwrpwrors.supabase.co';
const supabaseKey = 'sb_publishable_O2O2sqesYlfJoHMPSI8Zbg_kysXXK12';
const supabase = createClient(supabaseUrl, supabaseKey);

let canvas = document.getElementById("canvas");
canvas.width = window.innerWidth / 2;
canvas.height = window.innerHeight;
let ctx = canvas.getContext("2d");
let textArea = document.getElementById("text");
ctx.imageSmoothingEnabled = false;
let accounts = document.getElementById("account");
let userId2Input = document.getElementById("userId2")


let channel = null;

init().then(async () => {
	setHook();
	render();

	const {data: {user}} = await supabase.auth.getUser();
	if (user) {
		accounts.textContent = "Log Out";
		await supabase.from("Communication").delete().neq("id", 0);

		channel = supabase
			.channel("communication")
			.on(
				"postgres_changes",
				{
					event: "INSERT",
					schema: "public",
					table: "communication"
				},
				async function (payload) {
					console.log("New insert:", payload);
					if (payload.new.user_id2 === user.id) {
						handleDataIn(payload.new.message, payload.new.x, payload.new.y)
						await supabase.from("communication").delete().neq("id", 0);
					}
				}
			)
			.subscribe();
	} else {
		accounts.textContent = "Log In";
	}

	addEventListener("click", async function () {
		if ((await supabase.auth.getUser()).data) {
			await supabase.auth.signOut();
		}
		const {data, error} = await supabase.auth.signInWithOAuth({
			provider: 'discord',
			options: {
				redirectTo: window.location.origin
			}
		});
		if (error) {
			console.error("Login error:", error);
		}
	});

	document.getElementById("logOut")?.addEventListener("click", async function () {
		await supabase.auth.signOut();
	});

	document.addEventListener("keydown", async function (event) {
		handleKeyDown(event.key);
	});

	document.addEventListener("click", function (event) {
		handleMouseClick(event.x, event.y);
	});

	document.getElementById("beginConnection").addEventListener("click", function () {
		supabase
			.from("Communication")
			.insert({
				userId2: userId2Input.value,
				message: "Join",
				x: 0,
				y: 0
			})
	});

	textArea.addEventListener("data", function () {
		handleDataIn(textArea.value);
		textArea.value = "";
	});
});