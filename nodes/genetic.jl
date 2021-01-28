using PyCall

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



