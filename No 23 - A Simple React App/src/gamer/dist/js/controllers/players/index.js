"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.deletePlayer = exports.addPlayer = exports.getPlayers = void 0;
const player_1 = __importDefault(require("../../models/player"));
// Tüm oyuncu listesini çeker
const getPlayers = (req, res) => __awaiter(void 0, void 0, void 0, function* () {
    try {
        const players = yield player_1.default.find();
        res.status(200).json({ players });
    }
    catch (error) {
        throw error;
    }
});
exports.getPlayers = getPlayers;
// Yeni bir oyuncu eklemek için kullanılır
const addPlayer = (req, res) => __awaiter(void 0, void 0, void 0, function* () {
    try {
        console.log(req.body);
        // HTTP talebine ait gövdeden gerekli parametre değerleri IPlayer arayüzüne okunur
        const body = req.body;
        // Bu değerlerden yararlanarak yeni bir Player nesnesi örneklenir
        const player = new player_1.default({
            nickname: body.nickname,
            country: body.country,
            level: body.level,
        });
        // save metodu ile yeni Player mongodb'ye yazılır
        const newPlayer = yield player.save();
        res
            .status(201)
            .json({ message: "Yeni oyuncu eklendi", addedPlayer: newPlayer });
    }
    catch (error) {
        throw error;
    }
});
exports.addPlayer = addPlayer;
// Sistemden oyuncu silmek için kullanılır
const deletePlayer = (req, res) => __awaiter(void 0, void 0, void 0, function* () {
    try {
        const deleted = yield player_1.default.findByIdAndRemove(req.params.id);
        res.status(200).json({ message: "Oyuncu silindi", deletedPlayer: deleted });
    }
    catch (error) {
        throw error;
    }
});
exports.deletePlayer = deletePlayer;
