from flask import Flask, jsonify
from flask_restful import Resource, Api 
  
app = Flask(__name__) 

@app.route('/book')
def todaysbook():
    return jsonify(
        {'title': 'Learning Docker Compose'
        ,'description':'A Simple learning book about docker-compose tool'
        ,'publishdate':'2020'
        })
 