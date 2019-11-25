from flask import Flask
from flask_restful import Api, Resource, reqparse
from flask_cors import CORS


app = Flask(__name__)
CORS(app)
api = Api(app)

measurements = [
]

class Measurement(Resource):
    def post(self):
        parser = reqparse.RequestParser()
        parser.add_argument("date")
        parser.add_argument("temperature")
        args = parser.parse_args()

        for idx, m in enumerate(measurements):
            if(date == m["date"]):
                measurements[idx] = {
                    'date': args[date],
                    'temperature': args[temperature]
                }
                return measurements[idx], 201

        m = {
            "date": args['date'],
            "temperature": args["temperature"],
        }
        measurements.append(m)
        return m, 201


    def delete(self, date):
        global measurements
        measurements = [m for m in measurements if m["date"] != date]
        return "{} is deleted.".format(date), 200
      
class Measurements(Resource):
    def get(self):
        return measurements, 200



api.add_resource(Measurement, "/measurement")
api.add_resource(Measurements, "/measurements")
app.run(host='0.0.0.0', debug=True)