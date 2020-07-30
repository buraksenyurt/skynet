"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const mongoose_1 = require("mongoose");
const playerSchema = new mongoose_1.Schema({
    nickname: {
        type: String,
        required: true,
    },
    country: {
        type: String,
        required: true,
    },
    level: {
        type: Number,
        required: true,
    },
}, { timestamps: true });
exports.default = mongoose_1.model("Player", playerSchema);
