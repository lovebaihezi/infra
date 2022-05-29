module ml

import Flux
import Random
import Statistics
import Images

function distance(from::Vector{T}, to::Vector{V}) where {T,V}
    sum((from .- to) .^2) |> sqrt
end

function KMEANS(
    matrix::Vector{Vector{T}},
    k::Int;
    max::Int,
)::Vector{Vector{Vector{T}}} where {T}
    len = matrix |> length
    μ = Random.shuffle(Random.MersenneTwister(888888), 1:len)[1:k] .|> x -> matrix[x]
    C::Vector{Vector{Vector{T}}} = 1:k .|> _ -> []
    for _ ∈ 1:max
        for x ∈ matrix
            distances = μ .|> ϵ -> distance(x, ϵ)
            min = argmin(distances)
            C[min] = C[min] ∪ [x]
        end
        δ = C .|> x -> Statistics.mean(x)
        μ == δ && break
        μ = δ
    end
    C
end

main() = begin
    
end

end # module
