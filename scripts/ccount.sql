-- MySQL Script generated by MySQL Workbench
-- Sun 14 Mar 2021 03:22:07 PM EDT
-- Model: New Model    Version: 1.0
-- MySQL Workbench Forward Engineering

SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0;
SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0;
SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='ONLY_FULL_GROUP_BY,STRICT_TRANS_TABLES,NO_ZERO_IN_DATE,NO_ZERO_DATE,ERROR_FOR_DIVISION_BY_ZERO,NO_ENGINE_SUBSTITUTION';

-- -----------------------------------------------------
-- Schema ccount
-- -----------------------------------------------------

-- -----------------------------------------------------
-- Schema ccount
-- -----------------------------------------------------
CREATE SCHEMA IF NOT EXISTS `ccount` ;
USE `ccount` ;

-- -----------------------------------------------------
-- Table `ccount`.`user`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `ccount`.`user` (
  `email` VARCHAR(75) NOT NULL,
  `password` VARCHAR(250) NOT NULL,
  PRIMARY KEY (`email`),
  UNIQUE INDEX `email_UNIQUE` (`email` ASC) VISIBLE)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `ccount`.`category`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `ccount`.`category` (
  `id` BIGINT NOT NULL AUTO_INCREMENT,
  `name` VARCHAR(75) NOT NULL,
  PRIMARY KEY (`id`))
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `ccount`.`food`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `ccount`.`food` (
  `id` BIGINT NOT NULL AUTO_INCREMENT,
  `name` VARCHAR(75) NOT NULL,
  `calories` INT NOT NULL,
  `category_id` BIGINT NOT NULL,
  `user_email` VARCHAR(75) NOT NULL,
  PRIMARY KEY (`id`),
  INDEX `fk_food_category1_idx` (`category_id` ASC) VISIBLE,
  INDEX `fk_food_user1_idx` (`user_email` ASC) VISIBLE,
  CONSTRAINT `fk_food_category1`
    FOREIGN KEY (`category_id`)
    REFERENCES `ccount`.`category` (`id`)
    ON DELETE CASCADE
    ON UPDATE NO ACTION,
  CONSTRAINT `fk_food_user1`
    FOREIGN KEY (`user_email`)
    REFERENCES `ccount`.`user` (`email`)
    ON DELETE CASCADE
    ON UPDATE NO ACTION)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `ccount`.`daily_calories`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `ccount`.`daily_calories` (
  `id` BIGINT NOT NULL AUTO_INCREMENT,
  `day` VARCHAR(12) NOT NULL,
  `user_email` VARCHAR(75) NOT NULL,
  `food_id` BIGINT NOT NULL,
  PRIMARY KEY (`id`),
  INDEX `fk_daily_calories_user_idx` (`user_email` ASC) VISIBLE,
  INDEX `fk_daily_calories_food1_idx` (`food_id` ASC) VISIBLE,
  CONSTRAINT `fk_daily_calories_user`
    FOREIGN KEY (`user_email`)
    REFERENCES `ccount`.`user` (`email`)
    ON DELETE CASCADE
    ON UPDATE NO ACTION,
  CONSTRAINT `fk_daily_calories_food1`
    FOREIGN KEY (`food_id`)
    REFERENCES `ccount`.`food` (`id`)
    ON DELETE CASCADE
    ON UPDATE NO ACTION)
ENGINE = InnoDB;


SET SQL_MODE=@OLD_SQL_MODE;
SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS;
SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS;
