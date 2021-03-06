const assert = require('assert');
const Character = require("../../models/Character")

describe('Character', () => {

    describe("isAlive", () => {
        it("should return true if life > 0", () => {
            let char = new Character("Thessalonike", 8000);
            char.decreaseLifeBy(2000);
    
            assert.equal(char.isAlive(), true);
        });
        it("should return false if life <= 0", () => {
            let char = new Character("Thessalonike", 8000);
            char.decreaseLifeBy(9000);
    
            assert.rejects(char.isAlive());
        });
    });

    describe('decreaseLifeBy', () => {
        it('should reduce life by x if remaining > 0', () =>  {
            let char = new Character("Thessalonike", 8000);
            char.decreaseLifeBy(2000);

            assert.equal(char.getLife(), 6000);
        });

        it("should kill character when remaining < 0", () => {
            let char = new Character("Thessalonike", 8000);
            char.decreaseLifeBy(10000);

            assert.equal(char.getLife(), 0);
            assert.rejects(char.isAlive());
        });
    });

    describe("increaseLifeBy", () => {
        it("should increase life by x if remaining < life total", () => {
            let char = new Character("Thessalonike", 8000);
            char.decreaseLifeBy(4000);
            char.increaseLifeBy(1000);

            assert.equal(char.getLife(), 5000);
        });
        it("should restore life if remaining > life total", () => {
            let char = new Character("Thessalonike", 8000);
            char.decreaseLifeBy(4000);
            char.increaseLifeBy(10000);

            assert.equal(char.getLife(), char.getLifeTotal());
        });
    });

    describe("restoreLife", () => {
        it("should set life to lifeTotal", () => {
            let char = new Character("Thessalonike", 8000);
            char.decreaseLifeBy(4000);
            char.restoreLife();

            assert.equal(char.getLife(), char.getLifeTotal());
        });
    });

    describe("resetInitiative", () => {
        it("should set initiative to 0", () => {
            let char = new Character("Thessalonike", 8000, 25);
            char.resetInitiative();

            assert.equal(char.getInitiative(), 0);
        });
    });

    describe("restoreInitiative", () => {
        it("should set initiative to 200", () => {
            let char = new Character("Thessalonike", 8000, 25);
            char.restoreInitiative();

            assert.equal(char.getInitiative(), 200);
        });
    });

    describe("resetInitiative", () => {
        it("should set initiative to 0", () => {
            let char = new Character("Thessalonike", 8000, 25);
            char.resetInitiative();

            assert.equal(char.getInitiative(), 0);
        });
    });

    describe("increaseInitiative", () => {
        it("should increase initiative by speed", () => {
            let char = new Character("Thessalonike", 8000, 25);
            char.resetInitiative();
            char.increaseInitiative();

            assert.equal(char.getInitiative(), char.getSpeed());
        });
    });
    describe("increaseInitiative", () => {
        it("should set initiative to 200 when maxed", () => {
            let char = new Character("Thessalonike", 8000, 25);
            char.increaseInitiative();

            assert.equal(char.getInitiative(), 200);
        });
    });
});