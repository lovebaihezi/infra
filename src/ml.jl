module ml

# import Flux
import Random
import Statistics
import Images

function distance(from::Vector{T}, to::Vector{T}) where {T}
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

function indexes(index::Int64, width::Int64)::Vector{Int64}
    vcat(
        index - width * 2 - 2:index - width * 2 + 2,
        index - width * 1 - 2:index - width * 1 + 2,
        index - 2:index + 2,
        index + width * 1 - 2:index + width * 1 + 2,
        index + width * 2 - 2:index + width * 1 + 2,
    )
end

function main()
    K = 4
    image = Images.Gray.(Images.load("/home/lqxc/Lab/Medical/dicom.png"))
    arr = Images.colorview(Images.Gray, image)
    width, height = Images.width(image), Images.height(image)
    rands::Vector{Int32} = Random.shuffle(Random.MersenneTwister(12345678), 1:length(arr))[1:K]
    for y ∈ 0:height
        for x ∈ y * width + 1:(y + 1) * width
        end
    end
end

end # module

