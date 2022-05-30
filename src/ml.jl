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
)::Vector{Vector{Vector{T}}} where {T}
    len = matrix |> length
    μ = Random.shuffle(Random.MersenneTwister(888888), 1:len)[1:k] .|> x -> matrix[x]
    C::Vector{Vector{Vector{T}}} = 1:k .|> _ -> []
    while true
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
    K = 4
    image = Images.Gray.(Images.load("/home/lqxc/Lab/Medical/dicom.png"))
    arr = Images.colorview(Images.Gray, image)
    width, height = Images.width(image), Images.height(image)
    for y ∈ 1:10
        for x ∈ y * width:(y + 1) * width
            print(x, " ")
        end
        println()
    end
end

end # module
