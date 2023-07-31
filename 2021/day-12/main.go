package main

import (
	"bufio"
	"errors"
	"fmt"
	"log"
	"os"
	"strings"
)

func caveIsBig(cave string) bool {
	return cave == strings.ToUpper(cave)
}

func caveIsSmall(cave string) bool {
	return cave == strings.ToLower(cave)
}

type Route struct {
	Stops []string
	twice string
}

func (r *Route) GetKey() string {
	return strings.Join(r.Stops, ",")
}

func (r *Route) Next(stop string) Route {
	stops := make([]string, len(r.Stops))
	copy(stops, r.Stops)
	return Route{
		Stops: append(stops, stop),
        twice: r.twice,
	}
}

func (r *Route) GetStop() string {
	return r.Stops[len(r.Stops)-1]
}

func (r *Route) CurrentlyAt(stop string) bool {
	return r.GetStop() == stop
}

func newRoute(first, twice string) Route {
	return Route{
		Stops: []string{first},
		twice: twice,
	}
}

type Graph struct {
	Edges map[string][]string
}

func (g *Graph) Nodes ()[]string {
    nodes := map[string]bool{}
    for k, v := range g.Edges {
        nodes[k] = true
        for _, n := range v {
            nodes[n] = true
        }
    }
    result := []string{}
    for n := range nodes{
        result = append(result, n)
    }
    return result
}

func (g *Graph) Add(nodeFrom, nodeTo string) {
	if edge, ok := g.Edges[nodeFrom]; !ok {
		g.Edges[nodeFrom] = []string{nodeTo}
	} else {
		g.Edges[nodeFrom] = append(edge, nodeTo)
	}
}

func (g *Graph) Adjacents(node string) []string {
	if edge, ok := g.Edges[node]; !ok {
		return []string{}
	} else {
		return edge
	}
}

func readInput() (Graph, error) {
	graph := Graph{
		Edges: map[string][]string{},
	}
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		row := scanner.Text()
		parts := strings.Split(row, "-")
		if len(parts) != 2 {
			return Graph{}, errors.New("Bad data")
		}
		graph.Add(parts[0], parts[1])
		graph.Add(parts[1], parts[0])
	}
	if err := scanner.Err(); err != nil {
		return Graph{}, err
	}
	return graph, nil
}

type Pathfinder struct {
	Graph     *Graph
	CheckNext func(stop string, route *Route) bool
}

func (f *Pathfinder) Find(init []Route, end string) []Route {
	routes := map[string]Route{}
	queue := make([]Route, len(init))
    for i, r := range init {
        queue[i] = r
    }
	for len(queue) > 0 {
		route := queue[0]
		queue = queue[1:]

		// Finished
		if route.CurrentlyAt(end) {
			routes[route.GetKey()] = route
			continue
		}

		for _, adj := range f.Graph.Adjacents(route.GetStop()) {
			if f.CheckNext(adj, &route) {
				queue = append(queue, route.Next(adj))
			}
		}
	}
    result := []Route{}
    for _, r := range routes {
        result = append(result, r)
    }
	return result
}

func solvePartOne(graph *Graph) int {
	pf := Pathfinder{
		Graph: graph,
		CheckNext: func(stop string, route *Route) bool {
			// Can visit Big Caves
			if caveIsBig(stop) {
				return true
			}

			// Can visit Small Caves only once
			// Handling of start and end stops happened here
			for _, s := range route.Stops {
				// if caveIsSmall(s) && s == stop {
				if s == stop {
					return false
				}
			}

			// Fallback
			return true
		},
	}
    init := []Route{newRoute("start", "")}
	routes := pf.Find(init, "end")
	return len(routes)
}

func solvePartTwo(graph *Graph) int {
	pf := Pathfinder{
		Graph: graph,
		CheckNext: func(stop string, route *Route) bool {
			// Start is now available as next
			if stop == "start" {
				return false
			}

			// End is always available as next
			if stop == "end" {
				return true
			}

			// Can visit Big Caves anytime
			if caveIsBig(stop) {
				return true
			}

			// Can visit Small Caves only once
            canHaveSuchStopsInRoute := 0
            // But Cave marked as twice can be visited twice
            if stop == route.twice {
                canHaveSuchStopsInRoute = 1
            }
            count := 0
			for _, s := range route.Stops {
				if s == stop {
					count ++
				}
			}
            return count <= canHaveSuchStopsInRoute
		},
	}

    smalls := []string {}
    for _, stop := range graph.Nodes() {
        if stop == "start" || stop == "end" {
            continue
        }
        if caveIsSmall(stop) {
            smalls = append(smalls, stop)
        }
    }

    init := []Route{}
    for _, stop := range smalls {
        init = append(init, newRoute("start", stop))
    }

	routes := pf.Find(init, "end")

	return len(routes)
}

func main() {
	graph, err := readInput()
	if err != nil {
		log.Fatal(err)
	}

	var result int
	result = solvePartOne(&graph)
	fmt.Printf("Part one: %v\n", result)

	result = solvePartTwo(&graph)
	fmt.Printf("Part two: %v\n", result)
}
