// A script to quickly update the json schema


const fs = require("fs")
const path = require("path")

const fileData = fs.readFileSync(path.join(__dirname, "questionBank.json"), 'utf-8');

const questionTypes = Object.entries(JSON.parse(fileData));

const newQuestions = {
	behavioral: [],
	technical: {
		subtype: {}
	},
};

questionTypes.forEach(([key, value]) => {
	if(key === "behavioral"){
		newQuestions["behavioral"].push(value)
		return
	}

	const {technical} = newQuestions;

	if(!(key in technical)){
		technical["subtype"][key] = []
	}
	technical.subtype[key].push(value)
})

fs.writeFileSync("question_bank/questionBankTemp.json", JSON.stringify(newQuestions, null, 2));