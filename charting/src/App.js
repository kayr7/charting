import React, { useRef, useState, useEffect } from 'react';
import './App.css';
import * as d3 from 'd3';

import 'bootstrap/dist/css/bootstrap.min.css';
import DatePicker from "react-datepicker";
 
import "react-datepicker/dist/react-datepicker.css";

import Form from 'react-bootstrap/Form';
import Table from 'react-bootstrap/Table';


function jsonCopy(src) {
  return JSON.parse(JSON.stringify(src));
}

function isFloat(val) {
  var floatRegex = /^-?\d+(?:[.,]\d+)?$/;
  if (!floatRegex.test(val)) {
    console.log(val + " didn't pass the regex");
    return false;
  }

  val = parseFloat(val);
  if (isNaN(val))
      return false;
  return true;
}



class App extends React.Component {
  constructor(props) {
    super(props);
    this.state = { measurements: [
    ],
    temperature: "",
    date: new Date(),
    mittelschmerz: false,
    zwischenblutung: false,
    geschlechtsverkehr: false,
    schleimstruktur: "",
    blutung: "",
    };

    this.handleTemperatureChange = this.handleTemperatureChange.bind(this);
    this.handleDateChange = this.handleDateChange.bind(this);
//    this.handleMittelschmerzChange = this.handleMittelschmerzChange.bind(this);
//    this.handleZwischenblutungChange = this.handleZwischenblutungChange.bind(this);
//    this.handleGeschlechtsverkehrChange = this.handleGeschlechtsverkehrChange.bind(this);
//    this.handleSchleimstrukturChange = this.handleSchleimstrukturChange.bind(this);
//    this.handleBlutungChange = this.handleBlutungChange.bind(this);
    
    this.handleSubmit = this.handleSubmit.bind(this);

    this.handleGvUpdate = this.handleGvUpdate.bind(this);
    this.handleMittelschmerzUpdate = this.handleMittelschmerzUpdate.bind(this);
    this.handleZwischenblutungUpdate = this.handleZwischenblutungUpdate.bind(this);
    this.handleBlutungUpdate = this.handleBlutungUpdate.bind(this);
    this.handleSchleimstrukturUpdate = this.handleSchleimstrukturUpdate.bind(this);
    this.handleTemperatureUpdate = this.handleTemperatureUpdate.bind(this);

  }

  async handleSubmit(event) {
    var temps = jsonCopy(this.state.measurements);
//    console.log(this.state.date);
    let submitValue ={
      temperature: Number(this.state.temperature),
      date: this.state.date.toLocaleDateString('de'),
      mittelschmerz: this.state.mittelschmerz,
      zwischenblutung: this.state.zwischenblutung,
      geschlechtsverkehr: this.state.geschlechtsverkehr,
      schleimstruktur: this.state.schleimstruktur,
      blutung: this.state.blutung,
    };
    temps.push({submitValue});
    this.setState({measurements: temps});

    const response = await fetch('http://192.168.8.4:8001/measurement', {
      method: 'POST',
      headers: { 'Content-Type': 'text/plain' },
      body: JSON.stringify(submitValue),});
    await response.json();


    event.preventDefault();
  }


  componentDidMount() {
    fetch('http://192.168.8.4:8001/measurements')
    .then(result => {
      return result.json();
    }).then(dat => {
      this.setState({measurements: dat});
    })
  }


  handleTemperatureChange(event) {
//    if (!isFloat(event.target.value)) {
//      return;
//    }
    this.setState({temperature: event.target.value});
  }

  handleDateChange(date) {
    console.log(date.toLocaleDateString('de'));
    this.setState({date: date});
  }


  handleGvUpdate(event) {
    fetch('http://192.168.8.4:8001/update_gv', {
      method: 'POST',
      headers: { 'Content-Type': 'text/plain' },
      body: '{"date": "'+event.target.id+'", "value": '+event.target.checked+'}'})
      .then(result => {
        fetch('http://192.168.8.4:8001/measurements')
        .then(result => {
          return result.json();
        }).then(dat => {
          this.setState({measurements: dat});
        })
      })
  }
  handleZwischenblutungUpdate(event) {
    fetch('http://192.168.8.4:8001/update_zb', {
      method: 'POST',
      headers: { 'Content-Type': 'text/plain' },
      body: '{"date": "'+event.target.id+'", "value": '+event.target.checked+'}'})
      .then(result => {
        fetch('http://192.168.8.4:8001/measurements')
        .then(result => {
          return result.json();
        }).then(dat => {
          this.setState({measurements: dat});
        })
      })
  }

  handleBlutungUpdate(event) {
    fetch('http://192.168.8.4:8001/update_blutung', {
      method: 'POST',
      headers: { 'Content-Type': 'text/plain' },
      body: '{"date": "'+event.target.id+'", "value": "'+event.target.value+'"}'})
      .then(result => {
        fetch('http://192.168.8.4:8001/measurements')
        .then(result => {
          return result.json();
        }).then(dat => {
          this.setState({measurements: dat});
        })
      })
  }

  handleSchleimstrukturUpdate(event) {
    fetch('http://192.168.8.4:8001/update_schleim', {
      method: 'POST',
      headers: { 'Content-Type': 'text/plain' },
      body: '{"date": "'+event.target.id+'", "value": "'+event.target.value+'"}'})
      .then(result => {
        fetch('http://192.168.8.4:8001/measurements')
        .then(result => {
          return result.json();
        }).then(dat => {
          this.setState({measurements: dat});
        })
      })
  }

  
  
  handleTemperatureUpdate(event) {
    if (!isFloat(event.target.value)) {
      return;
    }
    console.log(event.target.value + " seems to be a valid input");

    fetch('http://192.168.8.4:8001/update_temperature', {
      method: 'POST',
      headers: { 'Content-Type': 'text/plain' },
      body: '{"date": "'+event.target.id+'", "value": '+parseFloat(event.target.value)+'}'})
      .then(result => {
        fetch('http://192.168.8.4:8001/measurements')
        .then(result => {
          return result.json();
        }).then(dat => {
          this.setState({measurements: dat});
        })
      })
  }



  handleMittelschmerzUpdate(event) {
    fetch('http://192.168.8.4:8001/update_ms', {
      method: 'POST',
      headers: { 'Content-Type': 'text/plain' },
      body: '{"date": "'+event.target.id+'", "value": '+event.target.checked+'}'})
      .then(result => {
        fetch('http://192.168.8.4:8001/measurements')
        .then(result => {
          return result.json();
        }).then(dat => {
          this.setState({measurements: dat});
        })
      })
  }


  render() {
    return (
      <div>
        <table>
          <tbody>
            <tr>
            <td>
            <Form onSubmit={this.handleSubmit}>
              <Form.Group>
                <Form.Label>Datum:</Form.Label>
                <DatePicker
                  selected={this.state.date}
                  onChange={this.handleDateChange}
                />
              </Form.Group>
              <Form.Group>
                <Form.Label>Temperatur</Form.Label>
                <Form.Control as="input" type="text" value={this.state.temperature} onChange={this.handleTemperatureUpdate} />
              </Form.Group>
              <input type="submit" value="Submit" />
            </Form>
            </td>
            <td>
            <Chart data={this.state.measurements}
             width={640}
             height={280}/>
            </td>
            </tr>
          </tbody>
        </table>

        <Table striped bordered hover>
          <thead>
            <tr>
              <th>Datum</th>
              <th>Temperatur</th>
              <th>Blutung</th>
              <th>Schleimstruktur</th>
              <th>Geschlechtsverkehr</th>
              <th>Mittelschmerz</th>
              <th>Zwischenblutung</th>
            </tr>
          </thead>
          <tbody>
            {
            
            Object.keys(this.state.measurements).reverse().map((k, i) => {
            let m = this.state.measurements[k];
            return (
              <tr key={i}>
                <td>{m.date}</td>
                <td>
                  <Form.Group>
                    <Form.Control as="input" 
                      type="text" 
                      id={m.date}
                      value={m.temperature} 
                      onChange={this.handleTemperatureUpdate} />
                  </Form.Group>
                </td>
                <td>
                <Form.Group>
                  <Form.Control as="select"
                    defaultValue={m.blutung}
                    id={m.date}
                    onChange={this.handleBlutungUpdate}>
                    <option></option>
                    <option>schmierblutung</option>
                    <option>wenig</option>
                    <option>mittel</option>
                    <option>stark</option>
                  </Form.Control>
                </Form.Group>
                </td>
                <td>
                <Form.Group>
                  <Form.Control as="select" 
                      onChange={this.handleSchleimstrukturUpdate}
                      id={m.date}
                      defaultValue={m.schleimstruktur}>
                    <option></option>
                    <option>cremig</option>
                    <option>spinnbar, durchsichtig</option>
                    <option>fluessig</option>
                  </Form.Control>
                </Form.Group>
                </td>
                <td>         
                  <Form.Check type="checkbox"
                              id={m.date}
                              onChange={this.handleGvUpdate}
                              defaultChecked ={m.geschlechtsverkehr}
                              />
                </td>
                <td>
                <Form.Check type="checkbox"
                              id={m.date}
                              onChange={this.handleMittelschmerzUpdate}
                              defaultChecked ={m.mittelschmerz}
                              />
                              </td>
                <td>
                <Form.Check type="checkbox"
                              id={m.date}
                              onChange={this.handleZwischenblutungUpdate}
                              defaultChecked ={m.zwischenblutung}
                              />
                </td>
              </tr>);})}
          </tbody>
        </Table>

      </div>

    )
  }  
}

const Chart = (props) => {
    let width = props.width;
    let height = props.height;
    const ref = useRef();

    const [data, setData] = useState(props.data);


    useEffect(() => {
      setData(props.data);
      let dates = data.map(obj => obj.date);
      let parseTime = d3.timeParse("%d.%m.%Y");

      let xExtent = d3.extent(dates, function(d) {return parseTime(d)});

      let gv = [];
      
      let blutungen = [];

      let blutungen_start = null;
      let blutungen_stop = null;
      let no_blutung = 0;
      data.forEach(element => {
        if (element.geschlechtsverkehr)
          gv.push(element);

        if (element.blutung.length > 0) {
          no_blutung = 0;
          if (blutungen_start == null) {
            blutungen_start = element.date;
            blutungen_stop = element.date;
          }
          else {
            blutungen_stop = element.date;
          }
          

        } else {
          if (blutungen_start != null)
            no_blutung++;
          if (no_blutung > 3 && blutungen_start != null) {
            blutungen.push([blutungen_start.slice(), blutungen_stop.slice()]);
            blutungen_stop = null;
            blutungen_start = null;
          }
        }
      });
      
      if (blutungen_start != null) {
        blutungen.push([blutungen_start.slice(), blutungen_stop.slice()]);
        blutungen_stop = null;
        blutungen_start = null;
      }


      const svg = d3.select(ref.current);
      svg.selectAll("*").remove();

      const xScale = d3.scaleTime()
        .domain(xExtent) 
        .range([0, width-50]);
      
        const yScale = d3.scaleLinear()
        .domain([35.8, 38.5])
        .range([height, 0]);  



      svg.append('g')
        .attr("transform", "translate(50, "+ (height+20) + ")")
        .call(d3.axisBottom(xScale))
      svg.append('g')
        .attr("transform", "translate(50, 20)")
        .attr("class", "grid")
        .call(d3.axisLeft(yScale)
              .ticks(20)
              .tickSize(-width));


      svg.selectAll('.blutungen')
        .data(blutungen)
        .enter().append("rect")
          .attr("fill", "red")
          .attr("transform", "translate(50, 20)")
          .attr("opacity", 0.2)
          .attr("x", function(d) { return xScale(parseTime(d[0]))})
          .attr("y", 0)
          .attr("width", function(d) {return xScale(parseTime(d[1])) - xScale(parseTime(d[0]))})
          .attr("height", height);


      svg.append('path')
        .datum(data)
        .attr("fill", "none")
        .attr("stroke", "steelblue")
        .attr("stroke-width", 1.5)
        .attr("transform", "translate(50,20)")
        .attr("d", d3.line()
          .x(function(d){return xScale(parseTime(d.date))})
          .y(function(d){return yScale(d.temperature)})
          .curve(d3.curveMonotoneX));
        
        svg.selectAll(".dot")
          .data(data)
        .enter().append("circle") // Uses the enter().append() method
          .attr("class", "dot") // Assign a class for styling
          .attr("transform", "translate(50,20)")
          .attr("cx", function(d) { return xScale(parseTime(d.date)) })
          .attr("cy", function(d) { return yScale(d.temperature) })
          .attr("r", 4);


        svg.selectAll(".dot2")
          .data(gv)
        .enter().append("text") // Uses the enter().append() method
//          .attr("class", "dot") // Assign a class for styling
          .attr("transform", "translate(43,10)")
          .attr("x", function(d) { return xScale(parseTime(d.date)) })
          .attr("y", function(d) { return yScale(d.temperature) })
          .text("❤");

          

    }, [data, height, width, props.data])

    return (
      <svg
        ref={ref}
        width='660'
        height='330'
      />
    )

  }

export default App;
