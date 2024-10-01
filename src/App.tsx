import { useState } from "react";

import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
	const [greetMsg, setGreetMsg] = useState("");
	const [message, setMessage] = useState("");
	const [response, setResponse] = useState("");
	const [name, setName] = useState("");

	async function greet() {
		// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
		setGreetMsg(await invoke("greet", { name }));
	}

	async function sendMessage(prompt: string) {
		try {
			const response = await invoke("generate_ai_response", { prompt });
			console.log("AI Response:", response);
			setResponse(response as string);
		} catch (error) {
			console.error("Error:", error);
		}
	}

	return (
		<div className="container">
			<h1>Welcome to Tauri!</h1>
			{/* <button type="button" onClick={() => sendMessage("Hello, how are you?")}>
				Send Message
			</button> */}

			<form
				className="row"
				onSubmit={(e) => {
					e.preventDefault();
					sendMessage(message);
				}}
			>
				<input
					id="greet-input"
					onChange={(e) => setMessage(e.currentTarget.value)}
					placeholder="Ask the ai..."
				/>
				<button type="submit">Send message</button>
			</form>
			<p>Ai response :</p>
			<p>{response}</p>
		</div>
	);
}

export default App;
