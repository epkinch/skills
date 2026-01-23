var express = require('express');
var router = express.Router();
var ctrlView1 = require('../controllers/view1');
var ctrlView2 = require('../controllers/view2');

/* View1 pages */
router.get('/', ctrlView1.task1);
router.get('/view1/task2', ctrlView1.task2);
router.get('/view1/task3', ctrlView1.task3);
/* View2 pages */
router.get('/view2', ctrlView2.task1);
module.exports = router;