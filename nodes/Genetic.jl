using PyCall
using Statistics
using StatsBase
using NLsolve

# Function to calculate distance between two points on a sphere
function haversine(long1::Float64, lat1::Float64, long2::Float64, lat2::Float64)
    RadiusEarth = 6378137 # This is the radius of earth in meters 
    σ1 = deg2rad(lat1)
    σ2 = deg2rad(lat2)
    δϕ = deg2rad(lat2 - lat1)
    δλ = deg2rad(long2 - long1)
    a = sin(δϕ / 2)^2  + cos(σ1) * cos(σ2) * sin(δλ / 2)^2
    c = 2 * atan.(sqrt(a), sqrt(1 -  a))
    length = RadiusEarth * c
    return length
end

# Function for the search elascticity between two nodes
function suffixheuristics(point1::Float32, point2::Float32)
    heuristic = exp((point1 + point2) / 2)
    return heuristic
end


# function to parse point data to Float64
function parsedata(pointing, position)
    a = pointing[position]["Geolocation"]
    m = chop(match(r"\(([^)]*)\)", a).match, head=1, tail=1)
    array = split(m, " ", limit=2)
    return parse(Float64, array[2]), parse(Float64, array[1])
end

function Normalize(array1)
    Columns, Rows = size(array1, 1), size(array1, 2)
    array3 = zeros((Columns, Rows))
    for j in 1:Rows
        for i in 1:Columns
            array3[j,i] = (array1[j,i] - minimum(array1[j,:])) / (maximum(array1[j,:]) - minimum(array1[j,:]))
        end
    end

    return array3
end

function Probability(PheromoneArray, CostArray)
    ProbaArray = zeros(size(PheromoneArray))
    for i in 1:size(PheromoneArray)[1]
        ProbaArray[i] =  (PheromoneArray[i] * CostArray[i]) / sum((PheromoneArray .* CostArray))
    end
    return ProbaArray
end

function AntColonyOptimisation(HeuristicsArray, AntsPopulation)
    Columns, Rows = size(HeuristicsArray, 1), size(HeuristicsArray, 2)
    PheromoneMatrix = ones((Columns, Rows))
    for i in 1:1000
        Colony = Vector{Matrix{Float64}}()
        AntPath = zeros((Columns, Rows))
        for i in 1:AntsPopulation
            push!(Colony, copy(AntPath))
        end
        for ant in Colony
            cost = 0
            locations = copy(collect(1:Rows))
            PathOfAnt = copy(PheromoneMatrix)
            edges = copy(HeuristicsArray)
            global OptPath = Vector{Int64}()
            for j in 1:Rows
                while true
                    circuit = size(locations)[1]
                    choice = sample(locations, Weights(Probability(PathOfAnt[j,:], edges[j,:])))
                    
                    push!(OptPath, choice)
                    position = findall(x -> x == choice, locations)
                    locations = filter!(x -> x ≠ choice, locations)
                    if circuit > size(locations)[1]
                        cost += edges[j,position, 1][1]
                        ant[j,position] .= 1
                        PathOfAnt = PathOfAnt[:, 1:end .!= position]
                        edges = edges[:, 1:end .!= position]
                        break
                    end
                end
            end
            PheromoneMatrix = PheromoneMatrix .+ (ant * cost)
        end
    end
    return OptPath
end

function LocalisePlaces(array1, array2)
    Nodes = Vector{Tuple{Float64,Float64}}()
    for i in array1
        push!(Nodes, parsedata(array2, i))
    end
    return Nodes
end

function LocaliseRegion(array1, array2)
    Nodes = Vector{String}()
    for i in array1
        push!(Nodes, array2[i]["Location"])
    end
    return Nodes
end

function initialMatrix(dict)
    HeuristicMatrix = Array{Float64}(undef, size(dict)[1], size(dict)[1])
    for (index, element) in enumerate(dict)
        for (needle, item) in enumerate(dict)
            if index == needle
                HeuristicMatrix[index,needle] = 0
            else 
                latitude1, longitude1 = parsedata(dict, index)
                latitude2, longitude2 = parsedata(dict, needle)
                Distance = haversine(longitude1, latitude1, longitude2, latitude2)
                GarbageAmount = suffixheuristics(convert(Float32, element["amount"]), convert(Float32, item["amount"]))
                HeuristicMatrix[index,needle] =  GarbageAmount + (1000 / Distance)
            end
        end
    end
    return LocalisePlaces(AntColonyOptimisation(Normalize(HeuristicMatrix), 200), dict), LocaliseRegion(AntColonyOptimisation(Normalize(HeuristicMatrix), 200), dict)
end

function binsearch(dict, position)
    HeuristicMatrix = Array{Float64}(undef, size(dict)[1], size(dict)[1])
    SuitableBin = Vector{Float64}()
    println(position)
    for (index, element) in enumerate(dict)
        latitude1, longitude1 = parsedata(dict, index)
        latitude2, longitude2 = position[1], position[2] 
        Distance = haversine(longitude1, latitude1, longitude2, latitude2)
        GarbageAmount = suffixheuristics(convert(Float32, element["amount"]), convert(Float32, 0.0))
        push!(SuitableBin, GarbageAmount * Distance)
    end
    return dict[argmin(SuitableBin)]["Location"]
end

