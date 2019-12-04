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
    this.handleMittelschmerzChange = this.handleMittelschmerzChange.bind(this);
    this.handleZwischenblutungChange = this.handleZwischenblutungChange.bind(this);
    this.handleGeschlechtsverkehrChange = this.handleGeschlechtsverkehrChange.bind(this);
    this.handleSchleimstrukturChange = this.handleSchleimstrukturChange.bind(this);
    this.handleBlutungChange = this.handleBlutungChange.bind(this);
    
    this.handleSubmit = this.handleSubmit.bind(this);

    this.handleGvUpdate = this.handleGvUpdate.bind(this);
    this.handleMittelschmerzUpdate = this.handleMittelschmerzUpdate.bind(this);
    this.handleZwischenblutungUpdate = this.handleZwischenblutungUpdate.bind(this);


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
    this.setState({temperature: event.target.value});
  }

  handleDateChange(date) {
    console.log(date.toLocaleDateString('de'));
    this.setState({date: date});
  }

  handleMittelschmerzChange(event) {
    this.setState({mittelschmerz: event.target.checked});
  }
  handleZwischenblutungChange(event) {
    this.setState({zwischenblutung: event.target.checked});
  }
  handleGeschlechtsverkehrChange(event) {
    this.setState({geschlechtsverkehr: event.target.checked});
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



  handleSchleimstrukturChange(event) {
    this.setState({schleimstruktur: event.target.value});
  }
  handleBlutungChange(event) {
    this.setState({blutung: event.target.value});
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
          <input type="text" value={this.state.temperature} onChange={this.handleTemperatureChange} />
        </Form.Group>
        <Form.Group>
          <Form.Label>Blutung</Form.Label>
          <Form.Control as="select" onChange={this.handleBlutungChange}>
            <option>--</option>
            <option>wenig</option>
            <option>mittel</option>
            <option>stark</option>
          </Form.Control>
        </Form.Group>
        <Form.Group>
          <Form.Label>Schleim</Form.Label>
          <Form.Control as="select" onChange={this.handleSchleimstrukturChange}>
            <option>--</option>
            <option>flockig, klebrig, wenig</option>
            <option>spinnbar, durchsichtig, fluessig</option>
          </Form.Control>
        </Form.Group>
        <Form.Group>
          <Form.Check type="checkbox" label="Mittelschmerz" onChange={this.handleMittelschmerzChange}/>
          <Form.Check type="checkbox" label="Zwischenblutung" onChange={this.handleZwischenblutungChange}/>
          <Form.Check type="checkbox" label="Geschlechtsverkehr" onChange={this.handleGeschlechtsverkehrChange}/>
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
            {Object.keys(this.state.measurements).map((k, i) => {
            let m = this.state.measurements[k];
            return (
              <tr key={i}>
                <td>{m.date}</td>
                <td>{m.temperature}</td>
                <td>{m.blutung}</td>
                <td>{m.schleimstruktur}</td>
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
      console.log(blutungen);


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
          .attr("x", function(d) { console.log("echo"); return xScale(parseTime(d[0]))})
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
