-- MySQL dump 10.13  Distrib 8.0.30, for Win64 (x86_64)
--
-- Host: 127.0.0.1    Database: ipfs_storage_cruster_manager
-- ------------------------------------------------------
-- Server version	8.0.30

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!50503 SET NAMES utf8mb4 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;

--
-- Table structure for table `node`
--

DROP TABLE IF EXISTS `node`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `node` (
  `id` varchar(100) NOT NULL,
  `peer_id` varchar(100) NOT NULL COMMENT 'ipfs peer id',
  `rpc_address` varchar(100) NOT NULL COMMENT 'Address of IPFS node''s rpc api',
  `wrapper_public_address` varchar(100) DEFAULT NULL COMMENT 'Address of node wrapper server (public)',
  `wrapper_admin_address` varchar(100) DEFAULT NULL COMMENT 'Address of node wrapper server (admin)',
  `node_status` enum('online','unhealthy','offline') NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `node_peer_id_uindex` (`peer_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='Bootstraped IPFS nodes'' metadata';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `node`
--

LOCK TABLES `node` WRITE;
/*!40000 ALTER TABLE `node` DISABLE KEYS */;
INSERT INTO `node` VALUES ('aaa','aaaa','www','cccc',NULL,'online');
/*!40000 ALTER TABLE `node` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `pin`
--

DROP TABLE IF EXISTS `pin`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `pin` (
  `id` varchar(100) NOT NULL COMMENT 'request id',
  `status` enum('Queued','Pinning','Pinned','Failed','NotFound') NOT NULL COMMENT 'pin status',
  `cid` varchar(100) NOT NULL COMMENT 'Pin CID',
  PRIMARY KEY (`id`),
  KEY `pin_cid_index` (`cid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='Pins';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `pin`
--

LOCK TABLES `pin` WRITE;
/*!40000 ALTER TABLE `pin` DISABLE KEYS */;
/*!40000 ALTER TABLE `pin` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `pins_stored_nodes`
--

DROP TABLE IF EXISTS `pins_stored_nodes`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `pins_stored_nodes` (
  `id` varchar(100) NOT NULL,
  `pin_id` varchar(100) NOT NULL,
  `node_id` varchar(100) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='Record which nodes stored the data of pin.';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `pins_stored_nodes`
--

LOCK TABLES `pins_stored_nodes` WRITE;
/*!40000 ALTER TABLE `pins_stored_nodes` DISABLE KEYS */;
/*!40000 ALTER TABLE `pins_stored_nodes` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `users_pins`
--

DROP TABLE IF EXISTS `users_pins`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `users_pins` (
  `id` varchar(100) NOT NULL,
  `user_id` varchar(100) NOT NULL COMMENT 'Id of the user',
  `pin_id` varchar(100) NOT NULL COMMENT 'Id of the pin',
  `pin_name` varchar(100) DEFAULT NULL COMMENT 'The name of pin given by a user',
  PRIMARY KEY (`id`),
  KEY `users_pins_user_id_index` (`user_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='Pins belong users';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `users_pins`
--

LOCK TABLES `users_pins` WRITE;
/*!40000 ALTER TABLE `users_pins` DISABLE KEYS */;
/*!40000 ALTER TABLE `users_pins` ENABLE KEYS */;
UNLOCK TABLES;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;

-- Dump completed on 2024-04-30 17:49:46
