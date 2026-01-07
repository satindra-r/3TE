import init, {
	setHook,
	render,
	handleKeyDown,
	handleMouseClick,
	handleDataIn,
} from "../pkg/LearningWASM.js";

import {createClient} from '@supabase/supabase-js';

const supabaseUrl = 'https://rnjrfxqopylbwrpwrors.supabase.co';
const supabaseKey = 'sb_publishable_O2O2sqesYlfJoHMPSI8Zbg_kysXXK12';
window.supabase = createClient(supabaseUrl, supabaseKey);

let canvas = document.getElementById("canvas");
canvas.width = window.innerWidth / 2;
canvas.height = window.innerHeight;
let ctx = canvas.getContext("2d");
ctx.imageSmoothingEnabled = false;
let accounts = document.getElementById("account");
let userId = document.getElementById("userId")
let userId2Input = document.getElementById("userId2")


let channel = null;

init().then(async () => {
	setHook();
	render();

	const {data: {user}} = await window.supabase.auth.getUser();
	if (user) {
		accounts.textContent = "Log Out";
		userId.textContent = "User Id:" + user.id + "📋";
		await window.supabase.from("Communication").delete().neq("id", 0);

		channel = window.supabase
			.channel("Communication")
			.on(
				"postgres_changes",
				{
					event: "INSERT",
					schema: "public",
					table: "Communication"
				},
				async function (payload) {
					console.log("New insert:", payload);
					if (payload.new.user_id2 === user.id) {
						handleDataIn(payload.new.message, payload.new.x, payload.new.y)
						await window.supabase.from("Communication").delete().neq("id", 0);
					}
				}
			)
			.subscribe();
	} else {
		accounts.textContent = "Log In";
	}

	accounts.addEventListener("click", async function () {
		if ((await window.supabase.auth.getUser()).data) {
			await window.supabase.auth.signOut();
			location.reload();

		}
		const {error} = await window.supabase.auth.signInWithOAuth({
			provider: 'discord',
			options: {
				redirectTo: window.location.href
			}
		});
		if (error) {
			console.error("Login error:", error);
		}
	});

	document.addEventListener("keydown", async function (event) {
		handleKeyDown(event.key);
	});

	document.addEventListener("click", function (event) {
		handleMouseClick(event.x, event.y);
	});

	userId.addEventListener("click", () => {
		navigator.clipboard.writeText(user.id);
	});
	document.getElementById("beginConnection").addEventListener("click", async function () {
		let player = 1;

		const {data: {user}} = await window.supabase.auth.getUser();
		const {data} = await supabase
			.from("Communication")
			.select("user_id, message, x, y")
			.eq("message", "Join")
			.eq("user_id2", user.id);
		if (data.length) {
			const {message, x, y} = data[0];
			handleDataIn(message, x, y);
			player = 3 - x;
		}
		await window.supabase
			.from("Communication")
			.insert({
				user_id: user.id,
				user_id2: userId2Input.value,
				message: "Join",
				x: player,
				y: 0
			}).select();
	});
});